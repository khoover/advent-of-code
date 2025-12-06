use anyhow::{Context, Result, anyhow};
use aoc_2025::run_day;
use itertools::Itertools;

fn part1(s: &str) -> Result<u64> {
    let mut lines = s.trim().lines();
    let op_types = lines
        .next_back()
        .context("Expected at least one line")?
        .split_whitespace()
        .map(|s| match s {
            "*" => Ok(OpType::Product),
            "+" => Ok(OpType::Sum),
            _ => Err(anyhow!("Unexpected op type")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    lines
        .map(|s| {
            s.split_whitespace()
                .map(|c| c.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
        })
        .reduce(|totals, row| {
            Ok(totals?
                .into_iter()
                .zip(row?.into_iter())
                .zip(op_types.iter().copied())
                .map(|((a, b), op)| match op {
                    OpType::Product => a * b,
                    OpType::Sum => a + b,
                })
                .collect())
        })
        .transpose()?
        .context("Expected at least one line")
        .map(|v| v.into_iter().sum())
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum OpType {
    Product,
    Sum,
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
*   +   *   + ";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 4277556);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 3263827);
}
