pub use nom::{
    branch::alt,
    bytes::complete::{
        tag, tag_no_case, take_till1, take_until, take_until1, take_while,
        take_while1,
    },
    character::complete::{
        i128 as I128, i8 as I8, multispace0, multispace1, one_of, space1,
        u8 as U8,
    },
    combinator::{
        all_consuming, cut, map, map_parser, opt, peek, rest, verify,
    },
    multi::{many0, many1, separated_list0, separated_list1},
    number::complete::{double, recognize_float},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

pub use anyhow::Context;

pub use log::*;
pub use std::hash::Hash;

pub use std::process::{Command, Stdio};

pub type Res<'a, T> = IResult<&'a str, T>;

pub use std::sync::Arc;

pub use std::ops::Deref;
pub use std::sync::{Mutex, MutexGuard};
pub use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

pub mod colors {
    pub use nu_ansi_term::Color::*;
    pub use nu_ansi_term::Style;
}
pub use serde::{Deserialize, Serialize};

pub const SHARED_LIB_DIR: &str = ".libs_adana";

pub fn get_path_to_shared_libraries() -> Option<PathBuf> {
    dirs::data_dir().or_else(dirs::home_dir).map(|mut pb| {
        pb.push(PathBuf::from(SHARED_LIB_DIR));
        pb
    })
}
