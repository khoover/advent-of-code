use crate::utils::*;
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::cell::Cell;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PipeTile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl PipeTile {
    fn crawl(self, pos: (usize, usize), prev_dir: Direction) -> ((usize, usize), Direction) {
        use Direction::*;
        use PipeTile::*;

        match (self, prev_dir) {
            (NorthEast, South) | (SouthEast, North) | (Horizontal, East) => {
                ((pos.0, pos.1 + 1), East)
            }
            (NorthEast, West) | (NorthWest, East) | (Vertical, North) => {
                ((pos.0 - 1, pos.1), North)
            }
            (Vertical, South) | (SouthWest, East) | (SouthEast, West) => {
                ((pos.0 + 1, pos.1), South)
            }
            (Horizontal, West) | (NorthWest, South) | (SouthWest, North) => {
                ((pos.0, pos.1 - 1), West)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct PipeGraph {
    column_count: usize,
    row_count: usize,
    starting_tile: (usize, usize),
    tiles: Vec<PipeTile>,
}

impl PipeGraph {
    fn from_input(input: &str) -> Self {
        let column_count = input.find('\n').unwrap();
        let mut row_count = 0;
        let starting_tile = Cell::new(None);
        let starting_tile_ref = &starting_tile;
        let mut tiles: Vec<PipeTile> = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                row_count += 1;
                line.as_bytes()
                    .iter()
                    .copied()
                    .enumerate()
                    .map(move |(col_idx, b)| match b {
                        b'S' => {
                            starting_tile_ref.set(Some((row_idx, col_idx)));
                            PipeTile::Ground
                        }
                        b'.' => PipeTile::Ground,
                        b'|' => PipeTile::Vertical,
                        b'-' => PipeTile::Horizontal,
                        b'F' => PipeTile::SouthEast,
                        b'7' => PipeTile::SouthWest,
                        b'J' => PipeTile::NorthWest,
                        b'L' => PipeTile::NorthEast,
                        _ => unreachable!(),
                    })
            })
            .collect();

        let starting_tile = starting_tile.get().unwrap();
        let starting_tile_idx = starting_tile.1 + column_count * starting_tile.0;
        let north = starting_tile.0 > 0
            && matches!(
                tiles[starting_tile_idx - column_count],
                PipeTile::Vertical | PipeTile::SouthEast | PipeTile::SouthWest
            );
        let south = starting_tile.0 < row_count - 1
            && matches!(
                tiles[starting_tile_idx + column_count],
                PipeTile::Vertical | PipeTile::NorthEast | PipeTile::NorthWest
            );
        let east = starting_tile.1 < column_count
            && matches!(
                tiles[starting_tile_idx + 1],
                PipeTile::Horizontal | PipeTile::NorthWest | PipeTile::SouthWest
            );
        let west = starting_tile.1 > 0
            && matches!(
                tiles[starting_tile_idx - 1],
                PipeTile::Horizontal | PipeTile::NorthEast | PipeTile::SouthEast
            );
        tiles[starting_tile_idx] = match (north, south, east, west) {
            (true, true, false, false) => PipeTile::Vertical,
            (true, false, true, false) => PipeTile::NorthEast,
            (true, false, false, true) => PipeTile::NorthWest,
            (false, true, true, false) => PipeTile::SouthEast,
            (false, true, false, true) => PipeTile::SouthWest,
            (false, false, true, true) => PipeTile::Horizontal,
            _ => unreachable!(),
        };

        Self {
            column_count,
            row_count,
            starting_tile,
            tiles,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<PipeTile> {
        (row < self.row_count && col < self.column_count)
            .then(|| self.tiles[col + row * self.column_count])
    }
}

#[aoc_generator(day10)]
fn day10_gen(input: &str) -> PipeGraph {
    PipeGraph::from_input(input)
}

#[aoc(day10, part1)]
fn part1(input: &PipeGraph) -> Result<usize> {
    let starting_pos = input.starting_tile;

    let mut tile = input
        .get(starting_pos.0, starting_pos.1)
        .context("Bad starting position")?;
    let mut pos = starting_pos;
    let mut prev_direction = match tile {
        PipeTile::Vertical | PipeTile::NorthEast | PipeTile::NorthWest => Direction::South,
        PipeTile::Horizontal | PipeTile::SouthEast => Direction::West,
        PipeTile::SouthWest => Direction::East,
        PipeTile::Ground => unreachable!(),
    };
    let mut steps = 0;
    loop {
        (pos, prev_direction) = tile.crawl(pos, prev_direction);
        tile = input.get(pos.0, pos.1).context("Broken crawl")?;
        steps += 1;
        if pos == starting_pos {
            break;
        }
    }

    Ok(steps / 2)
}

#[aoc(day10, part2)]
fn part2(input: &PipeGraph) -> Result<i64> {
    let starting_pos = input.starting_tile;

    let mut tile = input
        .get(starting_pos.0, starting_pos.1)
        .context("Bad starting position")?;
    let mut pos = starting_pos;
    let mut prev_direction = match tile {
        PipeTile::Vertical | PipeTile::NorthEast | PipeTile::NorthWest => Direction::South,
        PipeTile::Horizontal | PipeTile::SouthEast => Direction::West,
        PipeTile::SouthWest => Direction::East,
        PipeTile::Ground => unreachable!(),
    };
    let mut area: i64 = 0;
    let mut steps = 0;
    loop {
        (pos, prev_direction) = tile.crawl(pos, prev_direction);
        tile = input.get(pos.0, pos.1).context("Broken crawl")?;
        steps += 1;

        match (prev_direction, tile) {
            (Direction::West, PipeTile::Horizontal | PipeTile::SouthEast)
            | (Direction::North, PipeTile::SouthWest) => {
                area -= pos.0 as i64;
            }
            (Direction::East, PipeTile::Horizontal | PipeTile::NorthWest)
            | (Direction::South, PipeTile::NorthEast) => {
                area += pos.0 as i64 + 1;
            }
            _ => (),
        }

        if pos == starting_pos {
            break;
        }
    }

    Ok(area.abs() - steps as i64)
}
