use std::ops::Range;

use anyhow::Result;
use aoc_2025::byte_grid::Grid;
use aoc_2025::run_day;
use memchr::memchr2_iter;

fn part1(s: &str) -> Result<u64> {
    let grid = Grid::from_input_lines(s.lines())?;
    let op_types_and_ranges = get_op_types(&grid[grid.height() - 1]);
    Ok(op_types_and_ranges
        .into_iter()
        .map(|(col_range, op_type)| -> u64 {
            let it = (0..grid.height() - 1)
                .map(|i| parse_ascii_bytes(grid[i][col_range.clone()].iter()));
            match op_type {
                OpType::Product => it.product(),
                OpType::Sum => it.sum(),
            }
        })
        .sum())
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum OpType {
    Product,
    Sum,
}

fn get_op_types(line: &[u8]) -> Vec<(Range<usize>, OpType)> {
    let mut out = Vec::new();
    let final_start = memchr2_iter(b'+', b'*', line).reduce(|start, end| {
        let range = start..end - 1;
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
    let grid = Grid::from_input_lines(s.lines())?;
    let op_types_and_ranges = get_op_types(&grid[grid.height() - 1]);
    let row_range = 0..grid.height() - 1;

    Ok(op_types_and_ranges
        .into_iter()
        .map(|(col_range, op_type)| -> u64 {
            let it =
                col_range.map(|col| parse_ascii_bytes(grid.col_at_rows(col, row_range.clone())));
            match op_type {
                OpType::Product => it.product(),
                OpType::Sum => it.sum(),
            }
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
