use anyhow::{Context, Result};
use aoc_2025::run_day;

fn part1(s: &str) -> Result<u64> {
    let mut parts = s.trim().split("\n\n").collect::<Vec<_>>();
    let regions = parts.pop().unwrap();
    let boxes = parts
        .into_iter()
        .map(|part| {
            part.as_bytes()
                .iter()
                .copied()
                .filter(|&b| b == b'#')
                .count() as u64
        })
        .collect::<Vec<_>>();

    Ok(regions
        .lines()
        .filter(|&line| {
            let (dimensions, counts) = line.split_once(": ").unwrap();
            let (x, y) = dimensions.split_once("x").unwrap();
            let area = x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap();
            area >= counts
                .split_whitespace()
                .map(|count| count.parse::<u64>().unwrap())
                .zip(boxes.iter().copied())
                .map(|(count, size)| count * size)
                .sum()
        })
        .count() as u64)
}

fn part2(s: &str) -> Result<&'static str> {
    todo!()
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}
