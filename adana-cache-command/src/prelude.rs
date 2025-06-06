pub use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{
        tag, tag_no_case, take_till1, take_until, take_until1, take_while,
        take_while1,
    },
    character::complete::{multispace0, multispace1, space1},
    combinator::{cut, map, opt, rest, verify},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
pub type Res<'a, T> = IResult<&'a str, T>;
