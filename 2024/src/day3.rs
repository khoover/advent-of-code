use crate::utils::*;
use memchr::arch::all::packedpair::HeuristicFrequencyRank;
use memchr::memmem::{FinderBuilder, Prefilter};
use regex::Regex;

struct Emprical;

impl HeuristicFrequencyRank for Emprical {
    fn rank(&self, byte: u8) -> u8 {
        const TABLE: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 16, 17, 0, 15, 13, 15, 17, 20, 227, 221, 15, 16, 112, 16, 0, 15, 39, 58, 54,
            61, 61, 60, 63, 62, 57, 64, 17, 14, 17, 0, 15, 15, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 15, 16, 0, 0, 15, 1, 14, 9, 68, 15,
            0, 87, 1, 0, 0, 103, 104, 18, 55, 1, 0, 29, 14, 33, 90, 0, 87, 0, 14, 0, 16, 0, 15, 15,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        TABLE[byte as usize]
    }
}

#[aoc(day3, part1, Naive)]
pub fn part1_naive(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    regex
        .captures_iter(input)
        .map(|capture| {
            parse_3(capture.get(1).unwrap().as_str()) * parse_3(capture.get(2).unwrap().as_str())
        })
        .sum()
}

fn parse_3(s: &str) -> u32 {
    let b = s.as_bytes();
    b.iter()
        .copied()
        .map(|x| (x - b'0') as u32)
        .reduce(|acc, next| acc * 10 + next)
        .unwrap()
}

#[aoc(day3, part1, Opt)]
pub fn part1_opt(s: &str) -> u32 {
    let input = s.as_bytes();
    let mut i = 0;
    let mut sum = 0;
    while i < input.len() - 7 {
        if unsafe { *input.get_unchecked(i) } == b'm' {
            let (res, next_i) = unsafe { match_mul(i, input) };
            if let Some(v) = res {
                sum += v;
            }
            i = next_i;
        } else {
            i += 1;
        }
    }
    sum
}

#[aoc(day3, part1, Memchr)]
pub fn part1(s: &str) -> u32 {
    let input = s.as_bytes();

    let mul_finder = FinderBuilder::new()
        .prefilter(Prefilter::Auto)
        .build_forward_with_ranker(Emprical, "mul(");

    mul_finder
        .find_iter(&input[..input.len() - 7])
        .filter_map(|m_index| unsafe { match_mul_suffix(m_index, input) }.0)
        .sum()
}

/// # SAFETY
/// m_index < input.len() - 7
unsafe fn match_mul(m_index: usize, input: &[u8]) -> (Option<u32>, usize) {
    const MUL: u32 = 0x6D_75_6C_28_u32.swap_bytes();
    let loaded = u32::from_le_bytes(unsafe {
        [
            *input.get_unchecked(m_index),
            *input.get_unchecked(m_index + 1),
            *input.get_unchecked(m_index + 2),
            *input.get_unchecked(m_index + 3),
        ]
    });
    let diff = loaded.wrapping_sub(MUL);
    if diff != 0 {
        return (None, m_index + (diff.trailing_zeros() as usize >> 3));
    }

    match_mul_suffix(m_index, input)
}

/// # SAFETY
/// m_index < input.len() - 7
unsafe fn match_mul_suffix(m_index: usize, input: &[u8]) -> (Option<u32>, usize) {
    let mut a = {
        let d = *input.get_unchecked(m_index + 4);
        if !d.is_ascii_digit() {
            return (None, m_index + 4);
        } else {
            d - b'0'
        }
    } as u32;

    let mut i = m_index + 5;
    if unsafe { input.get_unchecked(i) }.is_ascii_digit() {
        a = a * 10 + (input[i] - b'0') as u32;
        i += 1;
    }
    if unsafe { input.get_unchecked(i) }.is_ascii_digit() {
        a = a * 10 + (input[i] - b'0') as u32;
        i += 1;
    }
    // i <= m_index + 7 < input.len()
    if unsafe { *input.get_unchecked(i) } != b',' {
        return (None, i);
    }
    let Some(d) = input.get(i + 1).filter(|d| d.is_ascii_digit()) else {
        return (None, i + 1);
    };
    let mut b = (*d - b'0') as u32;
    if let Some(d) = input.get(i + 2).filter(|d| d.is_ascii_digit()) {
        b = b * 10 + (d - b'0') as u32;
        if let Some(d) = input.get(i + 3).filter(|d| d.is_ascii_digit()) {
            b = b * 10 + (d - b'0') as u32;
            i += 2;
        } else {
            i += 1;
        }
    }
    if matches!(input.get(i + 2), Some(b')')) {
        (Some(a * b), i + 3)
    } else {
        (None, i + 2)
    }
}

#[aoc(day3, part2, Naive)]
pub fn part2_naive(input: &str) -> u32 {
    let regex = Regex::new(r"do\(\)|don't\(\)|mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    let mut enabled = true;
    regex
        .captures_iter(input)
        .map(|capture| {
            let full = &capture.get(0).unwrap().as_str()[..3];
            if full == "do(" {
                enabled = true;
                0
            } else if full == "don" {
                enabled = false;
                0
            } else if enabled {
                parse_3(capture.get(1).unwrap().as_str())
                    * parse_3(capture.get(2).unwrap().as_str())
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day3, part2, Opt)]
pub fn part2(s: &str) -> u32 {
    let input = s.as_bytes();
    let mut i = 0;
    let mut sum = 0;
    let mut enabled = true;
    while i < input.len() - 7 {
        let c = unsafe { *input.get_unchecked(i) };
        if c == b'm' && enabled {
            let (res, next_i) = unsafe { match_mul(i, input) };
            if let Some(v) = res {
                sum += v;
            }
            i = next_i;
        } else if c == b'd' {
            let (res, next_i) = unsafe { match_do_dont(i, input) };
            if let Some(b) = res {
                enabled = b;
            }
            i = next_i;
        } else {
            i += 1;
        }
    }
    sum
}

/// # Safety
/// d_index < input.len() - 7
unsafe fn match_do_dont(d_index: usize, input: &[u8]) -> (Option<bool>, usize) {
    const DO: u64 = 0x64_6F_28_29_00000000_u64.swap_bytes();
    const DONT: u64 = 0x64_6F_6E_27_74_28_29_00_u64.swap_bytes();
    let loaded = u64::from_le_bytes(unsafe {
        input
            .get_unchecked(d_index..d_index + 8)
            .try_into()
            .unwrap_unchecked()
    });
    let do_input_diff = loaded.wrapping_sub(DO).trailing_zeros();
    let dont_input_diff = loaded.wrapping_sub(DONT).trailing_zeros();
    if do_input_diff >= 4 * 8 {
        (Some(true), d_index + 4)
    } else if dont_input_diff >= 7 * 8 {
        (Some(false), d_index + 7)
    } else {
        (
            None,
            d_index + (do_input_diff.min(dont_input_diff) as usize >> 3),
        )
    }
}

#[aoc(day3, part2, Memchr)]
pub fn part2_memchr(s: &str) -> u32 {
    const DO_LEN: usize = 4;
    const DONT_LEN: usize = 7;

    let mut input = s.as_bytes();
    let mut max_search_idx = input.len() - 7;
    let mut sum: u32 = 0;

    let mul_finder = FinderBuilder::new()
        .prefilter(Prefilter::Auto)
        .build_forward_with_ranker(Emprical, "mul(");
    let dont_finder = FinderBuilder::new()
        .prefilter(Prefilter::Auto)
        .build_forward_with_ranker(Emprical, "don't()");
    let do_finder = FinderBuilder::new()
        .prefilter(Prefilter::Auto)
        .build_forward_with_ranker(Emprical, "do()");

    loop {
        let Some(next_dont) = dont_finder.find(&input[..max_search_idx]) else {
            return sum
                + mul_finder
                    .find_iter(&input[..max_search_idx])
                    .filter_map(|m_index| unsafe { match_mul_suffix(m_index, input) }.0)
                    .sum::<u32>();
        };
        sum += mul_finder
            .find_iter(&input[..next_dont])
            .filter_map(|m_index| unsafe { match_mul_suffix(m_index, input) }.0)
            .sum::<u32>();
        if let Some(idx) = do_finder.find(&input[next_dont + DONT_LEN..max_search_idx]) {
            input = &input[idx + DO_LEN..];
            if let Some(max) = input.len().checked_sub(7) {
                max_search_idx = max;
            } else {
                break;
            }
        } else {
            break;
        };
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = include_str!("../input/2024/day3.txt");

    #[test]
    fn generate_frequencies() {
        let mut table = [0_u32; 256];
        for byte in INPUT.as_bytes() {
            table[*byte as usize] += 1;
        }
        let min = table.iter().copied().filter(|x| *x != 0).min().unwrap();
        let max = table.iter().copied().max().unwrap();
        let scaling_factor = (max - min).div_ceil(256);
        println!("{scaling_factor}");
        print!("const TABLE: [u8; 256] = [");
        for entry in table {
            print!(
                "{}, ",
                entry
                    .checked_sub(min - 1)
                    .unwrap_or_default()
                    .div_ceil(scaling_factor)
            );
        }
        println!("];");
    }

    #[test]
    fn intuition_check() {
        let x = [1, 2, 3];
        println!("{:?}", &x[..2]);
        println!("{:?}", &x[2..2]); // empty is fine

        // println!("{:?}", &x[3..2]); // out of bounds
    }
}
