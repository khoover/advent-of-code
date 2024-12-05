use crate::utils::*;

use anyhow::{Error, Result};
use nom::{
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::opt,
    error::VerboseError,
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    Finish, IResult, Parser,
};

#[aoc(day5, part1)]
fn part1_base(s: &str) -> Result<u32> {
    let mut lut = [false; 100 * 100];

    let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
    let rest = terminated(
        fold_many1(
            terminated(parse_mapping, newline),
            || (),
            |(), (a, b)| {
                lut[(a as usize * 100) + b as usize] = true;
                ()
            },
        ),
        newline,
    )
    .parse(s.as_bytes())
    .finish()
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))?
    .0;

    assert!(lut.iter().copied().any(|x| x));

    fold_many1(
        terminated(separated_list1(tag(","), parse_digit_pair), opt(tag("\n"))),
        || 0_u32,
        move |acc, list| {
            debug!(list);
            for i in 0..list.len() {
                let a = list[i];
                for j in i..list.len() {
                    let b = list[j];
                    if lut[(b as usize * 100) + a as usize] {
                        return acc;
                    }
                }
            }
            let mid = list[list.len() / 2] as u32;
            debug!(mid);
            acc + mid
        },
    )
    .parse_complete(rest)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

#[aoc(day5, part2)]
fn part2_base(s: &str) -> Result<u32> {
    let mut lut = [false; 100 * 100];

    let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
    let rest = terminated(
        fold_many1(
            terminated(parse_mapping, newline),
            || (),
            |(), (a, b)| {
                lut[(a as usize * 100) + b as usize] = true;
                ()
            },
        ),
        newline,
    )
    .parse(s.as_bytes())
    .finish()
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))?
    .0;

    assert!(lut.iter().copied().any(|x| x));

    fold_many1(
        terminated(separated_list1(tag(","), parse_digit_pair), opt(tag("\n"))),
        || 0_u32,
        move |acc, mut list| {
            debug!(list);
            let mut was_wrong = false;
            let mut i = 0;
            while i < list.len() {
                let mut a = list[i];
                let mut j = i;
                while j < list.len() {
                    let b = list[j];
                    if lut[(b as usize) * 100 + a as usize] {
                        list.swap(i, j);
                        a = b;
                        was_wrong = true;
                        j = i;
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
            debug!(was_wrong);
            if was_wrong {
                let mid = list[list.len() / 2] as u32;
                debug!(mid);
                acc + mid
            } else {
                acc
            }
        },
    )
    .parse_complete(rest)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

fn parse_digit_pair(input: &[u8]) -> IResult<&[u8], u8, VerboseError<&[u8]>> {
    take::<usize, &[u8], VerboseError<&[u8]>>(2_usize)
        .map_opt(|pair: &[u8]| Some(pair[0].checked_sub(b'0')? * 10 + pair[1].checked_sub(b'0')?))
        .parse(input)
}

pub fn part1(s: &str) -> Result<u32> {
    part1_base(s)
}

pub fn part2(s: &str) -> Result<u32> {
    part2_base(s)
}

#[cfg(test)]
mod test {
    use super::*;

    static SITE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    const SITE_PART1_OUTPUT: u32 = 143;
    const SITE_PART2_OUTPUT: u32 = 123;

    static MY_INPUT: &str = include_str!("../input/2024/day5.txt");
    const MY_PART1_OUTPUT: u32 = 7024;

    #[test]
    fn test_part1_base_site() {
        assert_eq!(part1_base(SITE_INPUT).unwrap(), SITE_PART1_OUTPUT);
    }

    #[test]
    fn test_part1_base_mine() {
        assert_eq!(part1_base(MY_INPUT).unwrap(), MY_PART1_OUTPUT);
    }

    #[test]
    fn test_part2_base_site() {
        assert_eq!(part2_base(SITE_INPUT).unwrap(), SITE_PART2_OUTPUT);
    }
}
