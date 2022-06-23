pub use hashbrown::HashMap;
use nom::error::VerboseError;
pub use nom::{
    branch::alt,
    bytes::complete::{
        tag, tag_no_case, take_till1, take_until, take_until1, take_while,
        take_while1,
    },
    character::complete::{multispace0, multispace1, space1},
    combinator::{cut, eof, map, opt, rest, verify},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

pub use log::*;
pub use std::hash::Hash;
pub use std::process::{Command, Stdio};
pub use std::time::Duration;
pub use std::{collections::BTreeMap, io::stdout};

pub type Res<'a, T> = IResult<&'a str, T>;
pub type ResVerbose<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub use std::sync::Arc;

pub use std::sync::{Mutex, MutexGuard};
pub use std::ops::Deref;
pub use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    process::ExitCode,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod colors {
    pub use nu_ansi_term::Color::*;
    pub use nu_ansi_term::Style;
}
pub use serde::{Deserialize, Serialize};
