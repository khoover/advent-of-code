use super::*;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use rayon::{join, prelude::*};
use rustc_hash::FxHashMap;

#[aoc(day11, part1, Rayon)]
pub fn part1_rayon(input: &str) -> u64 {
    input
        .split_ascii_whitespace()
        .map(|s| <u64>::from_str_radix(s, 10).unwrap())
        .par_bridge()
        .map(|stone| calculate_stone_count(stone, 25))
        .sum()
}

fn calculate_stone_count(start: u64, remaining_blinks: u8) -> u64 {
    if remaining_blinks == 0 {
        1
    } else if start == 0 {
        calculate_stone_count(1, remaining_blinks - 1)
    } else {
        let digits = start.ilog10() + 1;
        if digits % 2 == 1 {
            // odd number of digits, multiply
            calculate_stone_count(start.checked_mul(2024).unwrap(), remaining_blinks - 1)
        } else {
            // If there are N digits, then we want start % (10 ^ {N/2}) and floor(start / (10 ^ {N/2}))
            let split = 10_u64.pow(digits / 2);
            let low = start % split;
            let high = start / split;
            let (low_count, high_count) = join(
                move || calculate_stone_count(low, remaining_blinks - 1),
                move || calculate_stone_count(high, remaining_blinks - 1),
            );
            low_count + high_count
        }
    }
}

#[aoc(day11, part1, Cache)]
pub fn part1_cache(input: &str) -> u64 {
    let mut cache: FxHashMap<(u64, u8), u64> = FxHashMap::default();
    input
        .split_ascii_whitespace()
        .map(|s| <u64>::from_str_radix(s, 10).unwrap())
        .map(|stone| calculate_stone_count_with_cache(stone, 25, &mut cache))
        .sum()
}

#[aoc(day11, part2, Cache)]
pub fn part2_cache(input: &str) -> u64 {
    let mut cache: FxHashMap<(u64, u8), u64> = FxHashMap::default();
    input
        .split_ascii_whitespace()
        .map(|s| <u64>::from_str_radix(s, 10).unwrap())
        .map(|stone| calculate_stone_count_with_cache(stone, 75, &mut cache))
        .sum()
}

fn calculate_stone_count_with_cache(
    start: u64,
    remaining_blinks: u8,
    cache: &mut FxHashMap<(u64, u8), u64>,
) -> u64 {
    if remaining_blinks == 0 {
        1
    } else {
        if let Some(count) = cache.get(&(start, remaining_blinks)) {
            return *count;
        }
        let res = if start == 0 {
            calculate_stone_count_with_cache(1, remaining_blinks - 1, cache)
        } else {
            let digits = start.ilog10() + 1;
            if digits % 2 == 1 {
                // odd number of digits, multiply
                calculate_stone_count_with_cache(
                    start.checked_mul(2024).unwrap(),
                    remaining_blinks - 1,
                    cache,
                )
            } else {
                // If there are N digits, then we want start % (10 ^ {N/2}) and floor(start / (10 ^ {N/2}))
                let split = 10_u64.pow(digits / 2);
                let low = start % split;
                let high = start / split;
                let low_count = calculate_stone_count_with_cache(low, remaining_blinks - 1, cache);
                let high_count =
                    calculate_stone_count_with_cache(high, remaining_blinks - 1, cache);
                low_count.checked_add(high_count).unwrap()
            }
        };
        cache.insert((start, remaining_blinks), res);
        res
    }
}
