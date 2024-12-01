use nom::{
    character::complete::space0,
    error::{ParseError, VerboseError},
    sequence::delimited,
    IResult, Parser,
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
