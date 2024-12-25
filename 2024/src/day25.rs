use super::*;

use itertools::{Either, Itertools};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Lock([u8; 5]);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Key([u8; 5]);

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let (keys, locks) = input
        .split("\n\n")
        .partition_map::<Vec<_>, Vec<_>, _, _, _>(|block| {
            let block = block.as_bytes();
            let actual_range = &block[6..36];
            if block[0] == b'#' {
                let heights: [u8; 5] = std::array::from_fn(|column| {
                    (0..5_u8)
                        .find(|row| actual_range[*row as usize * 6 + column] == b'.')
                        .unwrap_or(5)
                });
                Either::Right(heights)
            } else {
                let heights: [u8; 5] = std::array::from_fn(|column| {
                    5 - (0..5_u8)
                        .find(|row| actual_range[*row as usize * 6 + column] == b'#')
                        .unwrap_or(5)
                });
                Either::Left(heights)
            }
        });
    keys.into_iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| (0..5).all(|idx| key[idx] + lock[idx] <= 5))
                .count()
        })
        .sum()
}
