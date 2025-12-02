use anyhow::{Context, Result};
use aoc_2025::run_day;
use std::fmt::Write;

const POWERS_OF_10: [u64; 13] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
];

fn part1(s: &str) -> Result<u64> {
    let mut acc: u64 = 0;
    let mut buf = String::with_capacity(64);
    for range in s.trim().split(',') {
        let (a, b) = range
            .split_once("-")
            .with_context(|| format!("Invalid range: {}", range))?;
        let lower: u64 = a.parse()?;
        let upper: u64 = b.parse()?;
        let mut candidate_half: u64 = if a.len() % 2 == 1 {
            POWERS_OF_10[a.len() / 2]
        } else {
            let tmp = unsafe { std::str::from_utf8_unchecked(&a.as_bytes()[..a.len() / 2]) };
            tmp.parse()?
        };
        buf.clear();
        write!(&mut buf, "{}{}", candidate_half, candidate_half)?;
        let mut candidate: u64 = buf.parse()?;
        while candidate <= upper {
            if candidate >= lower {
                acc += candidate;
            }
            candidate_half += 1;
            buf.clear();
            write!(&mut buf, "{}{}", candidate_half, candidate_half)?;
            candidate = buf.parse()?;
        }
    }
    Ok(acc)
}

fn part2(s: &str) -> Result<u64> {
    let ranges = s
        .trim()
        .split(',')
        .map(|range| -> Result<(u64, u64)> {
            let (a, b) = range
                .split_once('-')
                .with_context(|| format!("Invalid range: {}", range))?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;

    Ok(ranges
        .into_iter()
        .flat_map(|(a, b)| a..=b)
        .filter(|&x| {
            if x == 0 {
                return false;
            }

            let n = x.ilog10() + 1;
            for k in 1..=n / 2 {
                if n.is_multiple_of(k) {
                    let reps = n / k;
                    let pow = POWERS_OF_10[k as usize];
                    let base = x % pow;
                    let mut acc = base;
                    for _ in 0..(reps - 1) {
                        acc = acc * pow + base;
                    }
                    if acc == x {
                        return true;
                    }
                }
            }
            false
        })
        .sum())
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 1227775554);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 4174379265);
}
