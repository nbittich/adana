use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, io::Write};

pub fn write_cursor_and_flush() {
    print!("> ");
    let _ = std::io::stdout().flush();
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
