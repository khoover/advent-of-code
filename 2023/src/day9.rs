use crate::utils::*;
use anyhow::Result;
use aoc_runner_derive::aoc;
use nom::{
    character::complete::{i64, space1},
    multi::separated_list1,
    Parser,
};

fn parse_coefficients(input: &str) -> StrIResult<'_, Vec<i64>> {
    separated_list1(space1, i64).parse(input)
}

#[aoc(day9, part1)]
fn part1(input: &str) -> Result<i64> {
    input
        .lines()
        .map(|line| run_parse(line, parse_coefficients))
        .map(|parse_res| parse_res.map(extrapolate_one))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> Result<i64> {
    input
        .lines()
        .map(|line| run_parse(line, parse_coefficients))
        .map(|parse_res| {
            parse_res.map(|mut seq| {
                seq.reverse();
                extrapolate_one(seq)
            })
        })
        .sum()
}

fn extrapolate_one(mut input_seq: Vec<i64>) -> i64 {
    let mut acc = 0;
    loop {
        let mut all_zeroes = true;
        for i in 0..input_seq.len() - 1 {
            all_zeroes &= input_seq[i] == 0;
            input_seq[i] = input_seq[i + 1] - input_seq[i];
        }
        let Some(res) = input_seq.pop().filter(|&v| v != 0 || !all_zeroes) else {
            break acc;
        };

        acc += res;
    }
}
