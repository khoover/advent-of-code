use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u32, u8},
    combinator::{all_consuming, map},
    multi::{fold_many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
    Finish, IResult, Parser,
};
use rustc_hash::FxHashSet;

struct Card {
    winners: FxHashSet<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn matching(&self) -> usize {
        self.numbers
            .iter()
            .filter(|x| self.winners.contains(*x))
            .count()
    }

    fn part1_value(&self) -> u64 {
        let count = self.matching();
        if count > 0 {
            1 << (count - 1)
        } else {
            0
        }
    }

    fn nom(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(pair(tag("Card"), space1), u32),
                tag(": "),
                card_contents,
            ),
            |(_, (winners, numbers))| Self { winners, numbers },
        )
        .parse(input)
    }
}

fn card_contents(input: &str) -> IResult<&str, (FxHashSet<u8>, Vec<u8>)> {
    separated_pair(
        winners,
        pair(tag(" |"), space1),
        separated_list1(space1, u8),
    )
    .parse(input)
}

fn winners(input: &str) -> IResult<&str, FxHashSet<u8>> {
    fold_many1(preceded(space0, u8), FxHashSet::default, |mut set, x| {
        set.insert(x);
        set
    })
    .parse(input)
}

#[aoc_generator(day4, part1, Struct)]
#[aoc_generator(day4, part2, Struct)]
fn day4_gen(input: &str) -> Result<Vec<Card>> {
    all_consuming(separated_list1(newline, Card::nom))
        .parse_complete(input)
        .finish()
        .map(|(_, x)| x)
        .map_err(|e| Error::msg(format!("Failed to parse input: {e}")))
}

#[aoc(day4, part1, Struct)]
fn day4_part1(input: &[Card]) -> u64 {
    input.iter().map(Card::part1_value).sum()
}

#[aoc(day4, part2, Struct)]
fn day4_part2(input: &[Card]) -> usize {
    let mut counts = vec![1; input.len()];
    input.iter().enumerate().for_each(|(idx, card)| {
        for copy_idx in (idx + 1)..(idx + card.matching() + 1) {
            counts[copy_idx] += counts[idx];
        }
    });
    counts.into_iter().sum()
}

#[aoc_generator(day4, part1, Counts)]
#[aoc_generator(day4, part2, Counts)]
fn day4_count_gen(input: &str) -> Result<Vec<usize>> {
    all_consuming(separated_list1(
        newline,
        preceded(
            (tag("Card"), space1, u32, tag(":")),
            winners.flat_map(|winning_set| {
                preceded(
                    tag(" | "),
                    fold_many1(
                        preceded(space0, u8),
                        || 0_usize,
                        move |count, num| {
                            if winning_set.contains(&num) {
                                count + 1
                            } else {
                                count
                            }
                        },
                    ),
                )
            }),
        ),
    ))
    .parse_complete(input)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input: {e}")))
}

#[aoc(day4, part1, Counts)]
fn day4_part1_count(input: &[usize]) -> u64 {
    input
        .iter()
        .copied()
        .map(|count| if count > 0 { 1 << (count - 1) } else { 0 })
        .sum()
}

#[aoc(day4, part2, Counts)]
fn day4_part2_count(input: &[usize]) -> usize {
    let mut counts = vec![1; input.len()];
    input.iter().copied().enumerate().for_each(|(idx, count)| {
        for copy_idx in (idx + 1)..(idx + count + 1) {
            counts[copy_idx] += counts[idx];
        }
    });
    counts.into_iter().sum()
}
