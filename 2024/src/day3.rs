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
