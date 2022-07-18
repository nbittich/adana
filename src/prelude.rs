pub use nom::{
    branch::alt,
    bytes::complete::{
        tag, tag_no_case, take_till1, take_until, take_until1, take_while,
        take_while1,
    },
    character::complete::{
        alpha1, alphanumeric1, i128 as I128, multispace0, multispace1, none_of,
        one_of, space1,line_ending
    },
    combinator::{all_consuming, cut, eof, map, map_parser, opt, rest, verify},
    multi::{many0, many1, separated_list0},
    number::complete::{double, recognize_float},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

pub use anyhow::Context;

pub use log::*;
pub use std::hash::Hash;

pub use std::io::stdout;
pub use std::process::{Command, Stdio};
pub use std::time::Duration;

pub type Res<'a, T> = IResult<&'a str, T>;

pub use std::sync::Arc;

pub use std::ops::Deref;
pub use std::sync::{Mutex, MutexGuard};
pub use std::{
    collections::BTreeMap,
    fs::File,
    io::BufReader,
    panic::AssertUnwindSafe,
    path::PathBuf,
    process::ExitCode,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod colors {
    pub use nu_ansi_term::Color::*;
    pub use nu_ansi_term::Style;
}
pub use serde::{Deserialize, Serialize};
