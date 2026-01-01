use std::ops::Range;

use anyhow::{Context, Result, anyhow};
use aoc_2025::byte_grid::Grid;
use aoc_2025::run_day;
use itertools::Itertools;
use memchr::memchr2_iter;

fn part1(s: &str) -> Result<u64> {
    let mut lines = s.lines();
    let op_types_and_ranges = get_op_types(
        lines
            .next_back()
            .context("Expected at least one line")?
            .as_bytes(),
    );
    let grid = Grid::from_input_lines(lines)?;
    let mut acc = op_types_and_ranges
        .iter()
        .map(|(_, op_type)| match op_type {
            OpType::Product => 1_u64,
            OpType::Sum => 0_u64,
        })
        .collect_vec();
    for row in (0..grid.height()).map(|i| &grid[i]) {
        for (acc, (range, op_type)) in acc.iter_mut().zip_eq(op_types_and_ranges.iter().cloned()) {
            let num = parse_ascii_bytes(&row[range]);
            match op_type {
                OpType::Product => *acc *= num,
                OpType::Sum => *acc += num,
            }
        }
    }
    Ok(acc.into_iter().sum())
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum OpType {
    Product,
    Sum,
}

fn get_op_types(line: &[u8]) -> Vec<(Range<usize>, OpType)> {
    let mut out = Vec::new();
    let final_start = memchr2_iter(b'+', b'*', line).reduce(|start, end| {
        let range = start..end;
        let op_type = if line[start] == b'*' {
            OpType::Product
        } else {
            OpType::Sum
        };
        out.push((range, op_type));
        end
    });
    if let Some(start) = final_start {
        let range = start..line.len();
        let op_type = if line[start] == b'*' {
            OpType::Product
        } else {
            OpType::Sum
        };
        out.push((range, op_type));
    }
    out
}

fn parse_ascii_bytes<'a>(it: impl IntoIterator<Item = &'a u8>) -> u64 {
    it.into_iter()
        .copied()
        .filter_map(|x| x.checked_sub(b'0').filter(|digit| *digit < 10))
        .fold(0_u64, |acc, digit| acc * 10 + digit as u64)
}

fn part2(s: &str) -> Result<u64> {
    let lines = s.trim().lines().collect::<Vec<_>>();
    let op_types = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| match s {
            "*" => Ok(OpType::Product),
            "+" => Ok(OpType::Sum),
            _ => Err(anyhow!("Unexpected op type")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut ops = op_types.into_iter();
    let mut utf8_buf: Vec<u8> = vec![0; lines.len() - 1];
    Ok((0..lines[0].len())
        .batching(|it| {
            let (op, idx) = ops.next().zip(it.next())?;

            lines.iter().zip(utf8_buf.iter_mut()).for_each(|(l, b)| {
                *b = l.as_bytes()[idx];
            });
            let mut acc = unsafe { std::str::from_utf8_unchecked(&utf8_buf) }
                .trim()
                .parse::<u64>()
                .unwrap();

            for idx in it {
                lines.iter().zip(utf8_buf.iter_mut()).for_each(|(l, b)| {
                    *b = l.as_bytes()[idx];
                });
                let s = unsafe { std::str::from_utf8_unchecked(&utf8_buf) }.trim();
                if s.is_empty() {
                    break;
                }
                let x = s.parse::<u64>().unwrap();
                match op {
                    OpType::Product => acc *= x,
                    OpType::Sum => acc += x,
                }
            }
            Some(acc)
        })
        .sum())
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 4277556);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 3263827);
}
