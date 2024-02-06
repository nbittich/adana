pub use std::{collections::BTreeMap, path::PathBuf};

pub mod colors {
    pub use nu_ansi_term::Color::*;
    pub use nu_ansi_term::Style;
}

pub const SHARED_LIB_DIR: &str = ".libs_adana";

pub fn get_path_to_shared_libraries() -> Option<PathBuf> {
    dirs::data_dir().or_else(dirs::home_dir).map(|mut pb| {
        pb.push(PathBuf::from(SHARED_LIB_DIR));
        pb
    })
}
