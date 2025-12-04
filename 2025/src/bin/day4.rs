use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc_2025::run_day;

fn part1(s: &str) -> Result<u64> {
    let lines: Vec<_> = s.trim().lines().map(|l| l.as_bytes()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut acc: u64 = 0;
    for row in 0..height {
        for col in 0..width {
            if lines[row][col] == b'@'
                && grid_iter(row, col, width, height)
                    .map(|(row, col)| if lines[row][col] == b'@' { 1 } else { 0 })
                    .sum::<u8>()
                    < 4
            {
                acc += 1;
            }
        }
    }
    Ok(acc)
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn grid_iter(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    OFFSETS.into_iter().filter_map(move |(a, b)| {
        let row = row.checked_add_signed(a).filter(|&x| x < height);
        let col = col.checked_add_signed(b).filter(|&x| x < width);
        row.zip(col)
    })
}

fn part2(s: &str) -> Result<u64> {
    let lines: Vec<_> = s.trim().lines().map(|l| l.as_bytes()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let to_linear = |row: usize, col: usize| -> usize { row * height + col };
    let mut state: Vec<u8> = lines
        .into_iter()
        .flat_map(|l| l.iter().copied().map(|b| if b == b'@' { 1 } else { 0 }))
        .collect();
    let mut to_flip: HashSet<(usize, usize)> = HashSet::new();
    let mut just_flipped: HashSet<(usize, usize)> = HashSet::new();
    let mut acc: u64 = 0;
    for row in 0..height {
        for col in 0..width {
            if state[to_linear(row, col)] == 1
                && grid_iter(row, col, width, height)
                    .map(|(row, col)| state[to_linear(row, col)])
                    .sum::<u8>()
                    < 4
            {
                to_flip.insert((row, col));
                acc += 1;
            }
        }
    }

    while !to_flip.is_empty() {
        for idx in to_flip.iter().copied().map(|(a, b)| to_linear(a, b)) {
            state[idx] = 0;
        }
        std::mem::swap(&mut to_flip, &mut just_flipped);
        to_flip.clear();
        just_flipped = just_flipped
            .into_iter()
            .flat_map(|(row, col)| grid_iter(row, col, width, height))
            .collect();
        for (row, col) in just_flipped.drain() {
            if state[to_linear(row, col)] == 1
                && grid_iter(row, col, width, height)
                    .map(|(row, col)| state[to_linear(row, col)])
                    .sum::<u8>()
                    < 4
            {
                to_flip.insert((row, col));
                acc += 1;
            }
        }
    }
    Ok(acc)
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 43);
}
