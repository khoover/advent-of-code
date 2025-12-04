use std::collections::VecDeque;

use anyhow::Result;
use aoc_2025::run_day;

fn part1(s: &str) -> Result<u64> {
    let lines: Vec<_> = s.trim().lines().map(|l| l.as_bytes()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut acc: u64 = 0;
    for row in 0..height {
        for col in 0..width {
            if lines[row][col] == b'@'
                && grid_iter(row, col, width, height)
                    .map(|(row, col)| if lines[row][col] == b'@' { 1 } else { 0 })
                    .sum::<u8>()
                    < 4
            {
                acc += 1;
            }
        }
    }
    Ok(acc)
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn grid_iter(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    OFFSETS.into_iter().filter_map(move |(a, b)| {
        let row = row.checked_add_signed(a).filter(|&x| x < height);
        let col = col.checked_add_signed(b).filter(|&x| x < width);
        row.zip(col)
    })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct TileState {
    has_wall: bool,
    neighbour_wall_count: u8,
}

impl TileState {
    fn is_flippable(self) -> bool {
        self.has_wall && self.neighbour_wall_count < 4
    }
}

fn part2(s: &str) -> Result<u64> {
    let lines: Vec<_> = s.trim().lines().map(|l| l.as_bytes()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let to_linear = |row: usize, col: usize| -> usize { row * height + col };

    let mut state: Vec<TileState> = lines
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(row, l)| {
            let line_ref = &lines;
            l.iter()
                .copied()
                .enumerate()
                .map(move |(col, b)| TileState {
                    has_wall: b == b'@',
                    neighbour_wall_count: grid_iter(row, col, width, height)
                        .filter(|&(row, col)| line_ref[row][col] == b'@')
                        .count() as u8,
                })
        })
        .collect();

    let mut to_flip: VecDeque<(usize, usize)> = VecDeque::new();
    for row in 0..height {
        for col in 0..width {
            if state[to_linear(row, col)].is_flippable() {
                to_flip.push_back((row, col));
            }
        }
    }

    let mut acc: u64 = 0;
    while let Some((row, col)) = to_flip.pop_front() {
        let idx = to_linear(row, col);
        if state[idx].is_flippable() {
            acc += 1;
            state[idx].has_wall = false;
            to_flip.extend(grid_iter(row, col, width, height).filter(|&(row, col)| {
                let idx = to_linear(row, col);
                state[idx].neighbour_wall_count -= 1;
                state[idx].has_wall && state[idx].neighbour_wall_count == 3
            }));
        }
    }
    Ok(acc)
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 43);
}
