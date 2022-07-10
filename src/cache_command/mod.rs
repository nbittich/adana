mod cache;
mod os_command;
mod parser;
mod process;
pub use cache::get_default_cache;
pub use process::process_command;

pub fn clear_terminal() {
    if cfg!(unix) {
        let _ = std::process::Command::new("clear").status();
    } else if cfg!(windows) {
        let _ = std::process::Command::new("cls").status();
    } else {
        eprintln!("cannot clear the terminal for the target os");
    };
}
