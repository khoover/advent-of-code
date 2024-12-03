use crate::utils::*;
use regex::Regex;

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
pub fn part1(s: &str) -> u32 {
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
