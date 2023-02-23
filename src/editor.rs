use anyhow::Context;
use log::debug;
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use rustyline::history::FileHistory;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{
    Cmd, CompletionType, Config, EditMode, Editor, KeyEvent, Movement,
};
use rustyline_derive::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::adana_script::constants::PI;

fn get_default_history_path() -> Option<Box<Path>> {
    let mut home_dir = dirs::home_dir()?;
    home_dir.push(".adana.history.txt");
    Some(home_dir.into_boxed_path())
}

pub fn save_history(
    rl: &mut Editor<CustomHelper, FileHistory>,
    history_path: Option<impl AsRef<Path>>,
) -> anyhow::Result<()> {
    let history_path = history_path
        .map(|p| p.as_ref().into())
        .or_else(get_default_history_path)
        .context("history path not found")?;
    rl.save_history(history_path.as_ref())
        .map_err(|e| anyhow::Error::msg(format!("{e}")))
}

pub fn read_line(
    rl: &mut Editor<CustomHelper, FileHistory>,
    curr_cache: &str,
) -> Result<String, rustyline::error::ReadlineError> {
    use nu_ansi_term::Color;
    let bold_white = Color::White.bold();
    let mut line = format!(
        "{}{}",
        bold_white.paint("["),
        Color::LightYellow.paint(curr_cache)
    );

    // show current dir & replace home dir by ~
    if let Ok(path) = std::env::current_dir() {
        let path = path.to_string_lossy().to_string();
        let home = dirs::home_dir()
            .unwrap_or(PathBuf::new())
            .to_string_lossy()
            .to_string();
        let path = path.replace(&home, "~");
        line += &format!("::{}", Color::LightBlue.paint(path));
    }

    // show current git branch
    let git_cmd = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output();
    match git_cmd {
        Ok(out) => {
            let branch = String::from_utf8_lossy(&out.stdout);
            let branch = branch.trim();
            if !branch.is_empty() {
                let branch = format!("({branch})");
                line += &format!("{}", Color::LightMagenta.paint(branch));
            }
        }
        Err(e) => {
            debug!(
                "Could not determine git current branch. Is git installed? {e}"
            );
        }
    }

    line += &format!("{} ", bold_white.paint("]"));

    rl.readline(&line)
}

pub fn build_editor(
    history_path: Option<impl AsRef<Path>>,
) -> Editor<CustomHelper, FileHistory> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .color_mode(rustyline::ColorMode::Enabled)
        .build();
    let h = CustomHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config)
        .expect("could not build editor with given config");
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::ctrl('d'), Cmd::Interrupt);
    rl.bind_sequence(KeyEvent::ctrl('c'), Cmd::Kill(Movement::WholeBuffer));
    rl.bind_sequence(KeyEvent::ctrl('l'), Cmd::ClearScreen);
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    rl.bind_sequence(KeyEvent::ctrl('x'), Cmd::Newline);
    rl.bind_sequence(KeyEvent::ctrl('p'), Cmd::Insert(1, format!("{PI} ")));
    if rl
        .load_history(
            history_path
                .map(|p| p.as_ref().into())
                .or_else(get_default_history_path)
                .context("history path missing")
                .unwrap()
                .as_ref(),
        )
        .is_err()
    {
        println!("No previous history.");
    }

    // let mut screen = stdout();
    //
    // std::thread::spawn(move || loop {
    //     //thread::sleep(Duration::from_millis(100));
    //     write!(screen, "\x1B[s\r{}\x1B[u", format_current_time(),).unwrap();
    //     screen.flush().unwrap();
    // });

    rl
}

// fn format_current_time() -> String {
//     let system_time = SystemTime::now();
//     let datetime: DateTime<Local> = system_time.into();
//     format!(
//         "{}",
//         nu_ansi_term::Color::White
//             .bold()
//             .paint(datetime.format("%T").to_string())
//     )
// }

#[derive(Helper, Completer, Highlighter, Validator, Hinter)]
pub struct CustomHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}
