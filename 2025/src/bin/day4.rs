use std::collections::VecDeque;

use anyhow::Result;
use aoc_2025::byte_grid::Grid;
use aoc_2025::run_day;

fn part1(s: &str) -> Result<u64> {
    let grid = Grid::from_input_str(s.trim())?;
    Ok(grid
        .enumerate()
        .filter(|(_, b)| **b == b'@')
        .filter(|&((row, col), _)| {
            grid.neighbours(row, col)
                .map(|coord| grid[coord])
                .filter(|&x| x == b'@')
                .count()
                < 4
        })
        .count() as u64)
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
    let grid = Grid::from_input_str(s.trim())?;
    let mut to_flip: VecDeque<(usize, usize)> = VecDeque::new();
    let mut state = grid.map(|(row, col), &b| {
        let state = TileState {
            has_wall: b == b'@',
            neighbour_wall_count: grid
                .neighbours(row, col)
                .filter(|&coord| grid[coord] == b'@')
                .count() as u8,
        };
        if state.is_flippable() {
            to_flip.push_back((row, col));
        }
        state
    });

    let mut acc: u64 = 0;
    while let Some((row, col)) = to_flip.pop_front() {
        if state[(row, col)].is_flippable() {
            acc += 1;
            state[(row, col)].has_wall = false;
            to_flip.extend(state.neighbours(row, col).filter(|&coord| {
                let neighbour = &mut state[coord];
                neighbour.neighbour_wall_count -= 1;
                neighbour.has_wall && neighbour.neighbour_wall_count == 3
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
