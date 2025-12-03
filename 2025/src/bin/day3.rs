use anyhow::{Context, Result};
use aoc_2025::run_day;

fn part1(s: &str) -> Result<u64> {
    Ok(s.trim()
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let (idx, tens_byte) = get_argmax(&bytes[..bytes.len() - 1]).unwrap();
            let (_, ones_byte) = get_argmax(&bytes[idx + 1..]).unwrap();
            (tens_byte - b'0') as u64 * 10 + (ones_byte - b'0') as u64
        })
        .sum())
}

fn get_argmax(arr: &[u8]) -> Option<(usize, u8)> {
    if arr.is_empty() {
        return None;
    }
    let mut idx: usize = 0;
    let mut max: u8 = b'0';
    for (i, val) in arr.iter().copied().enumerate() {
        if val > max {
            max = val;
            idx = i;
            if val == b'9' {
                break;
            }
        }
    }
    Some((idx, max))
}

fn part2(s: &str) -> Result<u64> {
    s.trim()
        .lines()
        .map(|line| {
            let mut view = line.as_bytes();
            let mut digits: [u8; 12] = [0; 12];
            for i in 0..12 {
                let (idx, byte) = get_argmax(&view[..view.len() - (11 - i)]).unwrap();
                digits[i] = byte;
                view = &view[idx + 1..];
            }
            unsafe { std::str::from_utf8_unchecked(&digits) }
                .parse::<u64>()
                .with_context(|| format!("Failed to parse {:?}", digits))
        })
        .sum()
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 357);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 3121910778619);
}
