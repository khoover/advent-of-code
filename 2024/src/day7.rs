use super::*;

use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, Parser,
};

fn parse_line(line: &str) -> StrIResult<(u64, Vec<u64>)> {
    separated_pair(u64, tag(": "), separated_list1(tag(" "), u64)).parse_complete(line)
}

#[aoc(day7, part1)]
fn part1(s: &str) -> u64 {
    s.lines()
        //.par_bridge()
        .map(|line| run_parse(line, parse_line).unwrap())
        .filter(|(target, inputs)| check_part1_line(*target, &*inputs))
        .map(|(target, _)| target)
        .sum()
}

fn check_part1_line(target: u64, inputs: &[u64]) -> bool {
    fn part1_recursion(target: u64, acc: u64, inputs: &[u64]) -> bool {
        if acc > target {
            return false;
        }
        let Some((first, rest)) = inputs.split_first() else {
            return target == acc;
        };

        part1_recursion(target, acc + *first, rest) || part1_recursion(target, acc * *first, rest)
    }

    let Some((first, rest)) = inputs.split_first() else {
        return false;
    };

    part1_recursion(target, *first, rest)
}

fn concat(a: u64, b: u64) -> u64 {
    let decimal_shift = b.ilog10();
    a * 10_u64.pow(decimal_shift + 1) + b
}

#[aoc(day7, part2)]
fn part2(s: &str) -> u64 {
    s.lines()
        .par_bridge()
        .map(|line| run_parse(line, parse_line).unwrap())
        .filter(|(target, inputs)| check_part2_line(*target, &*inputs))
        .map(|(target, _)| target)
        .sum()
}

fn check_part2_line(target: u64, inputs: &[u64]) -> bool {
    fn part2_recursion(target: u64, acc: u64, inputs: &[u64]) -> bool {
        if acc > target {
            return false;
        }
        let Some((first, rest)) = inputs.split_first() else {
            return target == acc;
        };

        part2_recursion(target, acc + *first, rest)
            || part2_recursion(target, acc * *first, rest)
            || part2_recursion(target, concat(acc, *first), rest)
    }

    let Some((first, rest)) = inputs.split_first() else {
        return false;
    };

    part2_recursion(target, *first, rest)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examine_input() {}
}
