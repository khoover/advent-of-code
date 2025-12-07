use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc_2025::{byte_grid::Grid, run_day};

fn part1(s: &str) -> Result<u64> {
    let grid = Grid::from_input_str(s.trim())?;
    let (start_row, start_col) = grid.find(b'S').context("Invalid input, missing start")?;
    let mut active_lasers: HashSet<usize> = HashSet::new();
    active_lasers.insert(start_col);
    let mut split_count: u64 = 0;

    for row in start_row + 1..grid.height() {
        let grid_row = &grid[row];
        active_lasers = active_lasers
            .into_iter()
            .flat_map(|col| {
                if grid_row[col] == b'^' {
                    split_count += 1;
                    [col.wrapping_sub(1), col + 1]
                } else {
                    [col, col]
                }
            })
            .filter(|col| *col < grid.width())
            .collect();
    }

    Ok(split_count)
}

fn part2(s: &str) -> Result<u64> {
    let grid = Grid::from_input_str(s.trim())?;
    let (start_row, start_col) = grid.find(b'S').context("Invalid input, missing start")?;
    let mut curr = vec![0_u64; grid.width()];
    let mut next = vec![0_u64; grid.width()];
    curr[start_col] = 1;
    for row_idx in start_row + 1..grid.height() {
        let grid_row = &grid[row_idx];
        for col_idx in 0..grid.width() {
            if grid_row[col_idx] == b'^' {
                if let Some(x) = next.get_mut(col_idx.wrapping_sub(1)) {
                    *x += curr[col_idx];
                }
                if let Some(x) = next.get_mut(col_idx + 1) {
                    *x += curr[col_idx];
                }
            } else {
                next[col_idx] += curr[col_idx];
            }
        }
        std::mem::swap(&mut curr, &mut next);
        next.fill(0);
    }
    Ok(curr.into_iter().sum())
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 40);
}
