use aoc_runner_derive::aoc;
use regex;
use std::sync::LazyLock;
use trie_rs::map::{Trie, TrieBuilder};

#[aoc(day1, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let left = line.chars().filter_map(|c| c.to_digit(10)).next().unwrap() * 10;
            let right = line
                .chars()
                .rev()
                .filter_map(|c| c.to_digit(10))
                .next()
                .unwrap();
            left as u64 + right as u64
        })
        .sum()
}

#[aoc(day1, part2, Replace)]
fn part2_replace(input: &str) -> u64 {
    let replacements = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    input
        .lines()
        .map(|line| {
            let line = replacements
                .iter()
                .fold(line.to_string(), |s, (a, b)| s.replace(a, b));
            let left = line.chars().filter_map(|c| c.to_digit(10)).next().unwrap() * 10;
            let right = line
                .chars()
                .rev()
                .filter_map(|c| c.to_digit(10))
                .next()
                .unwrap();
            left as u64 + right as u64
        })
        .sum()
}

#[aoc(day1, part2, Regex)]
fn part2_regex(input: &str) -> u64 {
    static START_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap()
    });
    static END_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"^.*(?P<number>[0-9]|one|two|three|four|five|six|seven|eight|nine)")
            .unwrap()
    });
    static TRIE: LazyLock<Trie<u8, u64>> = LazyLock::new(|| {
        let mut builder = TrieBuilder::new();
        builder.push("1", 1);
        builder.push("one", 1);
        builder.push("2", 2);
        builder.push("two", 2);
        builder.push("three", 3);
        builder.push("3", 3);
        builder.push("4", 4);
        builder.push("four", 4);
        builder.push("five", 5);
        builder.push("5", 5);
        builder.push("6", 6);
        builder.push("six", 6);
        builder.push("seven", 7);
        builder.push("7", 7);
        builder.push("eight", 8);
        builder.push("8", 8);
        builder.push("nine", 9);
        builder.push("9", 9);
        builder.build()
    });

    input
        .lines()
        .map(|line| {
            let first = START_REGEX.find(line).unwrap().as_str();
            let last = END_REGEX
                .captures(line)
                .unwrap()
                .name("number")
                .unwrap()
                .as_str();
            let first = *TRIE.exact_match(first).unwrap() * 10;
            let last = *TRIE.exact_match(last).unwrap();
            first + last
        })
        .sum()
}
