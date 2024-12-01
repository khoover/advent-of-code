use anyhow::{Error, Result};
use nom::{
    character::complete::space0,
    error::{ParseError, VerboseError},
    sequence::delimited,
    Finish, IResult, Parser,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(space0, inner, space0)
}

pub type StrIResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub fn run_parse<'a, O, P>(input: &'a str, mut parser: P) -> Result<O>
where
    P: Parser<&'a str, Output = O>,
    <P as Parser<&'a str>>::Error: std::fmt::Debug,
{
    parser
        .parse_complete(input)
        .finish()
        .map(|(_, x)| x)
        .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}
