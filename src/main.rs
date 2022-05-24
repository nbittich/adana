#![feature(let_chains, btree_drain_filter, exitcode_exit_method)]

mod cache;
mod os_command;
mod parser;
mod prelude;
mod utils;

use cache::CacheManager;
use nom::error::ErrorKind;
use os_command::exec_command;
pub use parser::{parse_command, CacheCommand};
pub use prelude::*;
use utils::write_cursor_and_flush;

lazy_static::lazy_static! {
    static ref CONFIG_FILE_PATH: PathBuf = {
        let mut conf_dir = dirs::config_dir().expect("conf dir not found");
        conf_dir.push(".karsher.conf.json");
        conf_dir
    };

    static ref CACHE_MANAGER: Arc<Mutex<CacheManager>> = {
        if let Ok(f) = File::open(CONFIG_FILE_PATH.as_path()) {
             let reader = BufReader::new(f);
             if let Ok(cache_manager) = serde_json::from_reader(reader) {
                 return Arc::new(Mutex::new(cache_manager));
             }
        }
     Default::default()
    };
}

fn main() -> anyhow::Result<()> {
    let mut current_cache = String::from("DEFAULT");

    let exit_lock = setup_ctrlc_handler(Arc::clone(&*CACHE_MANAGER));

    write_cursor_and_flush();
    let stdin = std::io::stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("read error");
        process_command(
            Arc::clone(&*CACHE_MANAGER),
            &mut current_cache,
            &line,
            Arc::clone(&exit_lock),
        )?;
        write_cursor_and_flush();
    }
}

fn setup_ctrlc_handler(cache_manager: Arc<Mutex<CacheManager>>) -> Arc<AtomicBool> {
    let lock = Arc::new(AtomicBool::new(true));
    let clone = Arc::clone(&lock);
    ctrlc::set_handler(move || {
        if !clone.load(Ordering::Relaxed) {
            println!("received Ctrl+C!");
        } else {
            if let Ok(cache_manager) = cache_manager.lock()  &&  let Ok(json) = serde_json::to_string_pretty(&*cache_manager) {
                if  std::fs::write(CONFIG_FILE_PATH.as_path(), json).is_ok() {
                    ExitCode::SUCCESS.exit_process();
                }else {
                    eprintln!("could not write to target conf file. gomenasai");
                }
            }else {
                eprintln!("could not acquire lock or could not serialize to json. sorry! bye.");
            }
            ExitCode::FAILURE.exit_process();
        }
    })
    .expect("Error setting Ctrl-C handler");
    lock
}

fn process_command(
    cache_manager: Arc<Mutex<CacheManager>>,
    current_cache: &mut String,
    line: &str,
    exit_lock: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let mut cache_manager = cache_manager
        .lock()
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    match parse_command(line) {
        Ok((_, command)) => match command {
            CacheCommand::Add { aliases, value } => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) {
                    let key = cache.insert(aliases, value);
                    println!("added {value} with hash key {key}");
                }
            },
            CacheCommand::Remove(key) => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(v) = cache.remove(key) {
                    println!("removed {v} with hash key {key}");
                }else {
                    println!("key {key} not found in current cache {current_cache}");
                }
            },
            CacheCommand::Get(key) => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(value) = cache.get(key) {
                    println!("found {value}");
                } else {
                    println!("{key} not found");
                }
            },
            CacheCommand::Exec(key) => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(value) = cache.get(key) {
                   exit_lock.store(false, Ordering::Relaxed);
                   let _ = exec_command(value).map_err(|e| anyhow::Error::msg(e.to_string()))?;
                   exit_lock.store(true, Ordering::Relaxed);

                } else {
                    println!("{key} not found");
                }
            },
            CacheCommand::Using(key) => {
                current_cache.clear();
                current_cache.push_str(key);
                println!("current cache: {key}");
            },
            CacheCommand::Dump(key) => {
                if let Some(key) = key {
                        if let Some(cache) = cache_manager.get(key) {
                               let cache = serde_json::to_string_pretty(&cache)?;
                               println!("{cache}")
                        }else{
                            eprintln!("cache {key} not found");
                        }
                } else{
                    let caches = serde_json::to_string_pretty(&*cache_manager)?;
                    println!("{caches}")
                }
            },
            CacheCommand::List => {
                if let Some(cache) = cache_manager.get(current_cache){
                    for ele in cache.list() {
                        println!("> {ele}");
                    }
                }
            }

        },
        Err(e) => {
            match e {
                nom::Err::Failure(failure) if failure.code == ErrorKind::Verify => {
                    eprintln!("invalid command")
                },
                _ => eprintln!("error parsing command: {e}")
            }
        },
    }
    Ok(())
}
