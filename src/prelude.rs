pub use nom::{
    branch::alt,
    bytes::complete::{
        tag, tag_no_case, take_till1, take_until, take_until1, take_while, take_while1,
    },
    character::complete::{multispace0, multispace1, space1},
    combinator::{cut, eof, map, opt, rest, verify},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
pub use std::process::{Command, Stdio};
pub use std::{
    collections::{BTreeMap, HashMap},
    io::stdout,
};

pub type Res<'a, T> = IResult<&'a str, T>;

pub use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    process::ExitCode,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
