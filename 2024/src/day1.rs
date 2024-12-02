use crate::utils::*;
use aoc_runner_derive::aoc;
use nom::{
    character::complete::{space1, u32},
    sequence::separated_pair,
};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| run_parse(line, separated_pair(u32, space1, u32)).unwrap())
        .unzip();

    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b) as u64)
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let mut list = Vec::new();
    let mut lookup = vec![0_u64; 100_000];
    input
        .lines()
        .map(|line| run_parse(line, separated_pair(u32, space1, u32)).unwrap())
        .for_each(|(left, right)| {
            list.push(left as usize);
            lookup[right as usize] += 1;
        });

    list.into_iter().map(|num| lookup[num] * num as u64).sum()
}
