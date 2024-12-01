use super::utils::{ws, StrIResult};
use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0, space1, u16},
    combinator::all_consuming,
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair},
    Finish, Parser,
};

fn get_races(input: &str) -> StrIResult<'_, Vec<(u16, u16)>> {
    separated_pair(
        preceded(tag("Time:"), ws(separated_list1(space1, u16))),
        newline,
        preceded(tag("Distance:"), ws(separated_list1(space1, u16))),
    )
    .map(|(times, distances)| {
        times
            .into_iter()
            .zip(distances.into_iter())
            .collect::<Vec<_>>()
    })
    .parse(input)
}

#[aoc_generator(day6, part1)]
fn day6_gen(input: &str) -> Result<Vec<(u16, u16)>> {
    all_consuming(get_races)
        .parse_complete(input)
        .finish()
        .map(|(_, x)| x)
        .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

fn quadratic_formula_width(time: u64, distance: u64) -> usize {
    let a = -1.0_f64;
    let b = time as f64;
    let c = -(distance as f64);
    let sqrt = (b * b - 4.0 * a * c).sqrt();
    let right = (-b - sqrt) / (2.0 * a);
    let left = (-b + sqrt) / (2.0 * a);
    (right.floor() - left.ceil()) as usize + 1
}

#[aoc(day6, part1)]
fn day6_part1(input: &[(u16, u16)]) -> usize {
    input
        .iter()
        .copied()
        .map(|(time, distance)| quadratic_formula_width(time as u64, distance as u64))
        .product()
}

fn kerned_number(input: &str) -> StrIResult<'_, u64> {
    fold_many1(
        preceded(space0, digit1),
        || 0_u64,
        |acc, new_digits: &str| {
            new_digits
                .as_bytes()
                .iter()
                .copied()
                .map(|b| b - b'0')
                .fold(acc, |inner_acc, digit| inner_acc * 10 + (digit as u64))
        },
    )
    .parse(input)
}

#[aoc_generator(day6, part2)]
fn day6_gen_part2(input: &str) -> Result<(u64, u64)> {
    all_consuming(separated_pair(
        preceded(tag("Time:"), ws(kerned_number)),
        newline,
        preceded(tag("Distance:"), ws(kerned_number)),
    ))
    .parse_complete(input)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

#[aoc(day6, part2)]
fn day6_part2(input: &(u64, u64)) -> usize {
    quadratic_formula_width(input.0, input.1)
}
