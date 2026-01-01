use anyhow::Result;
use aoc_2025::run_day;
use itertools::Itertools;

fn part1(s: &str) -> Result<u64> {
    let mut lines = s.trim().lines();
    let ranges: Vec<_> = make_ranges(lines.by_ref().take_while(|l| !l.is_empty())).collect();
    Ok(lines
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|id| match ranges.binary_search_by_key(id, |(a, _)| *a) {
            Ok(idx) => ranges[idx].0 <= *id && *id <= ranges[idx].1,
            Err(0) => false,
            Err(idx) => ranges[idx - 1].0 <= *id && *id <= ranges[idx - 1].1,
        })
        .count() as u64)
}

// Also sorted by increasing start
fn make_ranges<'a>(lines: impl IntoIterator<Item = &'a str>) -> impl Iterator<Item = (u64, u64)> {
    lines
        .into_iter()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .sorted_unstable()
        .coalesce(|(start, end), (next_start, next_end)| {
            if start <= next_start && next_start <= end {
                Ok((start, end.max(next_end)))
            } else {
                Err(((start, end), (next_start, next_end)))
            }
        })
}

fn part2(s: &str) -> Result<u64> {
    Ok(make_ranges(s.trim().lines().take_while(|l| !l.is_empty()))
        .map(|(a, b)| b - a + 1)
        .sum())
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 14);
}
