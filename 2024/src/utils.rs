use anyhow::{Error, Result};
pub use aoc_runner_derive::{aoc, aoc_generator};
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
    P: Parser<&'a str, Output = O, Error = VerboseError<&'a str>>,
{
    parser
        .parse_complete(input)
        .finish()
        .map(|(_, x)| x)
        .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

macro_rules! debug {
    ($x:tt) => { debug!(@ $x, $x);};
    (hex $x:tt) => { debug!(@x $x, $x);};
    (@ $xi:ident, $xe:expr) => {
        #[cfg(debug_assertions)]
        {
            print!(stringify!($xi));
            println!("={:?}", $xe);
        }
    };
    (@x $xi:ident, $xe:expr) => {
        #[cfg(debug_assertions)]
        {
            print!(stringify!($xi));
            println!("={:x?}", $xe);
        }
    }
}

pub(crate) use debug;

#[inline(always)]
pub fn arr_eq<T: PartialEq, const N: usize>(arr: &[T; N], c: T) -> [bool; N] {
    let mut out = [false; N];
    for i in 0..N {
        out[i] = arr[i] == c;
    }
    out
}

#[inline(always)]
pub fn arr_and<const N: usize>(a: [bool; N], b: [bool; N]) -> [bool; N] {
    let mut out = [false; N];
    for i in 0..N {
        out[i] = a[i] & b[i];
    }
    out
}

#[inline(always)]
pub fn arr_or<const N: usize>(a: [bool; N], b: [bool; N]) -> [bool; N] {
    let mut out = [false; N];
    for i in 0..N {
        out[i] = a[i] | b[i];
    }
    out
}
