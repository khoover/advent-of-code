use crate::utils::*;
use nom::{
    character::complete::{i16, space0},
    combinator::iterator,
    error::VerboseError,
    sequence::terminated,
};

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let mut it = iterator(line, terminated(i16::<&str, VerboseError<&str>>, space0));
            let first = it.next().unwrap();
            let mut diffs = it.scan(first, |prev_value, next_value| {
                let res = Some(next_value - *prev_value);
                *prev_value = next_value;
                res
            });
            let first_diff = diffs.next().unwrap();
            if !(1..=3).contains(&first_diff.abs()) {
                return false;
            }
            let sign = first_diff.signum();
            diffs.all(|diff| diff.signum() == sign && (1..=3).contains(&diff.abs()))
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let mut it = iterator(line, terminated(i16::<&str, VerboseError<&str>>, space0));
            let first = it.next().unwrap();
            let mut diffs = it.scan(first, |prev_value, next_value| {
                let res = Some(next_value - *prev_value);
                *prev_value = next_value;
                res
            });
            let mut ignored_bad = false;
            let mut first_diff = diffs.next().unwrap();
            if !(1..=3).contains(&first_diff.abs()) {
                ignored_bad = true;
                first_diff = diffs.next().unwrap();
                if !(1..=3).contains(&first_diff.abs()) {
                    return false;
                }
            }
            let sign = first_diff.signum();
            diffs.all(|diff| {
                (diff.signum() == sign && (1..=3).contains(&diff.abs()))
                    || !std::mem::replace(&mut ignored_bad, true)
            })
        })
        .count()
}
