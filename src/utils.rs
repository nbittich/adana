use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn clear_terminal() {
    if cfg!(unix) {
        let _ = std::process::Command::new("clear").status();
    } else if cfg!(windows) {
        let _ = std::process::Command::new("cls").status();
    } else {
        eprintln!("cannot clear the terminal for the target os");
    };
}

pub fn pid_exists(pid: u32) -> bool {
    Path::new(&format!("/proc/{}", pid)).exists()
}
