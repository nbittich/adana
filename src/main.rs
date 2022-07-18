mod args;
mod cache_command;
mod calculator;
mod db;
mod editor;
mod prelude;

use args::*;
use calculator::Primitive;
use db::DbOp;
use rustyline::error::ReadlineError;
use std::path::Path;

use prelude::{colors::LightBlue, colors::Style, debug, warn, BTreeMap};

use crate::{
    cache_command::{clear_terminal, get_default_cache, process_command},
    db::{Config, Db},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    env_logger::init();
    // trap SIGINT when CTRL+C for e.g with docker-compose logs -f
    ctrlc::set_handler(|| debug!("receive ctrl+c signal 2"))?;

    let args = parse_args(std::env::args())?;

    clear_terminal();
    println!("{PKG_NAME} v{VERSION}");

    let config = if args.is_empty() {
        Config::default()
    } else {
        let in_memory = args.iter().any(|a| matches!(a, Argument::InMemory));
        let fallback_in_memory =
            args.iter().any(|a| matches!(a, Argument::FallbackInMemory));
        let db_path = args.iter().find_map(|a| {
            if let Argument::DbPath(path) = a {
                Some(path)
            } else {
                None
            }
        });
        Config::new(db_path, in_memory, fallback_in_memory)
    };

    let history_path = args.iter().find_map(|a| {
        if let Argument::HistoryPath(path) = a {
            Some(path)
        } else {
            None
        }
    });

    println!();

    match Db::open(config) {
        Ok(Db::InMemory(mut db)) => start_app(&mut db, history_path),
        Ok(Db::FileBased(mut db)) => start_app(&mut db, history_path),
        Err(e) => Err(e),
    }
}

fn start_app(
    db: &mut impl DbOp<String, String>,
    history_path: Option<impl AsRef<Path> + Copy>,
) -> anyhow::Result<()> {
    let mut current_cache = {
        get_default_cache(db).as_ref().map_or("DEFAULT".into(), |v| v.clone())
    };
    let mut rl = editor::build_editor(history_path);
    let mut math_ctx = BTreeMap::new();
    loop {
        let readline = editor::read_line(&mut rl, &current_cache);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match process_repl(&line, &mut math_ctx) {
                    Ok(()) => (),
                    Err(e) => {
                        warn!("{e}");
                        process_command(db, &mut current_cache, &line)?;
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                std::process::exit(1);
            }
        }
    }

    editor::save_history(&mut rl, history_path)?;

    println!("{}", Style::new().bold().fg(LightBlue).paint("BYE"));
    Ok(())
}

fn process_repl(
    line: &str,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<()> {
    let calc = crate::calculator::compute(line, ctx)?;
    println!("{calc}");
    Ok(())
}
