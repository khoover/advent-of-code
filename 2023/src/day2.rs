use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    combinator::{all_consuming, map_res, opt, value},
    multi::{fold_many_m_n, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated},
    Finish, IResult,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

impl Draw {
    fn nom(input: &str) -> IResult<&str, Self> {
        map_res(
            fold_many_m_n(
                1,
                3,
                terminated(separated_pair(u32, space1, Rgb::nom), opt(tag(", "))),
                || Ok((None, None, None)),
                |tuple, (count, color)| match (tuple, color) {
                    (Ok((None, g, b)), Rgb::R) => Ok((Some(count), g, b)),
                    (Ok((r, None, b)), Rgb::G) => Ok((r, Some(count), b)),
                    (Ok((r, g, None)), Rgb::B) => Ok((r, g, Some(count))),
                    (Err(e), _) => Err(e),
                    _ => Err("same colour appeared multiple times"),
                },
            ),
            |res| {
                res.map(|(r, g, b)| Draw {
                    red: r.unwrap_or_default(),
                    green: g.unwrap_or_default(),
                    blue: b.unwrap_or_default(),
                })
            },
        )(input)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Rgb {
    R,
    G,
    B,
}

impl Rgb {
    fn nom(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::R, tag("red")),
            value(Self::G, tag("green")),
            value(Self::B, tag("blue")),
        ))(input)
    }
}

fn game_id(input: &str) -> IResult<&str, u32> {
    preceded(pair(tag("Game"), space1), u32)(input)
}

fn draws(input: &str) -> IResult<&str, Vec<Draw>> {
    separated_list1(tag("; "), Draw::nom)(input)
}

#[aoc_generator(day2)]
fn part1_gen(input: &str) -> Result<Vec<(u32, Vec<Draw>)>> {
    input
        .lines()
        .map(|line| {
            all_consuming(separated_pair(game_id, tag(": "), draws))(line)
                .finish()
                .map(|(_, x)| x)
                .map_err(|e| Error::msg(format!("Failed to parse input: {e}")))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(u32, Vec<Draw>)]) -> u32 {
    input
        .iter()
        .filter_map(|pair| {
            pair.1
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
                .then_some(pair.0)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(u32, Vec<Draw>)]) -> u64 {
    input
        .iter()
        .map(|(_, draws)| {
            let (r, g, b) = draws.iter().fold((0, 0, 0), |(r, g, b), draw| {
                (r.max(draw.red), g.max(draw.green), b.max(draw.blue))
            });
            r as u64 * g as u64 * b as u64
        })
        .sum()
}
