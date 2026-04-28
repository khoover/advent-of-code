use std::{
    cell::Cell,
    ops::{Index, IndexMut},
};

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> (isize, isize) {
        match value {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Robot,
    Box,
    Wall,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile2 {
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
    Empty,
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    contents: Vec<T>,
    robot_position: (isize, isize),
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.contents[index.0 * self.width + index.1]
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        &self[(index.0 as usize, index.1 as usize)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.contents[index.0 * self.width + index.1]
    }
}

impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        &mut self[(index.0 as usize, index.1 as usize)]
    }
}

#[aoc_generator(day15, part1)]
fn day15_gen(s: &str) -> (Grid<Tile>, Vec<Direction>) {
    let mut parts = s.split("\n\n");
    let grid = parts.next().unwrap();
    let directions = parts.next().unwrap();

    let width = grid.find("\n").unwrap();
    let height = (grid.len() + 1) / (width + 1);
    let robot_position = Cell::new(None);
    let contents = grid
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let borrowed_pos = &robot_position;
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(column, b)| match *b {
                    b'#' => Tile::Wall,
                    b'@' => {
                        assert!(borrowed_pos.get().is_none());
                        borrowed_pos.set(Some((row as isize, column as isize)));
                        Tile::Robot
                    }
                    b'.' => Tile::Empty,
                    b'O' => Tile::Box,
                    _ => unreachable!(),
                })
        })
        .collect();
    let grid = Grid {
        width,
        height,
        contents,
        robot_position: robot_position.get().unwrap(),
    };

    let directions = directions
        .as_bytes()
        .iter()
        .filter_map(|b| match *b {
            b'\n' => None,
            b'^' => Some(Direction::Up),
            b'>' => Some(Direction::Right),
            b'v' => Some(Direction::Down),
            b'<' => Some(Direction::Left),
            _ => unreachable!(),
        })
        .collect();

    (grid, directions)
}

#[aoc(day15, part1)]
pub fn part1(input: &(Grid<Tile>, Vec<Direction>)) -> u64 {
    let mut grid = input.0.clone();
    for direction in input.1.iter().copied() {
        let offset: (isize, isize) = direction.into();
        let mut search_pos = grid.robot_position;
        search_pos.0 += offset.0;
        search_pos.1 += offset.1;
        let next_robot_pos = search_pos;
        let can_move = loop {
            match grid[search_pos] {
                Tile::Empty => break true,
                Tile::Box => {
                    search_pos.0 += offset.0;
                    search_pos.1 += offset.1;
                }
                Tile::Wall => break false,
                Tile::Robot => unreachable!(),
            }
        };
        if can_move {
            grid[search_pos] = Tile::Box;
            let curr_pos = grid.robot_position;
            grid[curr_pos] = Tile::Empty;
            grid[next_robot_pos] = Tile::Robot;
            grid.robot_position = next_robot_pos;
        }
    }

    let mut sum = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid[(row, col)] == Tile::Box {
                sum += row as u64 * 100 + col as u64;
            }
        }
    }
    sum
}

#[aoc_generator(day15, part2)]
fn day15_gen_p2(s: &str) -> (Grid<Tile2>, Vec<Direction>) {
    let mut parts = s.split("\n\n");
    let grid = parts.next().unwrap();
    let directions = parts.next().unwrap();

    let width = grid.find("\n").unwrap();
    let height = (grid.len() + 1) / (width + 1);
    let robot_position = Cell::new(None);
    let contents = grid
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let borrowed_pos = &robot_position;
            line.as_bytes()
                .iter()
                .enumerate()
                .flat_map(move |(column, b)| match *b {
                    b'#' => [Tile2::Wall, Tile2::Wall],
                    b'@' => {
                        assert!(borrowed_pos.get().is_none());
                        borrowed_pos.set(Some((row as isize, 2 * column as isize)));
                        [Tile2::Robot, Tile2::Empty]
                    }
                    b'.' => [Tile2::Empty, Tile2::Empty],
                    b'O' => [Tile2::BoxLeft, Tile2::BoxRight],
                    _ => unreachable!(),
                })
        })
        .collect();
    let grid = Grid {
        width: width * 2,
        height,
        contents,
        robot_position: robot_position.get().unwrap(),
    };

    let directions = directions
        .as_bytes()
        .iter()
        .filter_map(|b| match *b {
            b'\n' => None,
            b'^' => Some(Direction::Up),
            b'>' => Some(Direction::Right),
            b'v' => Some(Direction::Down),
            b'<' => Some(Direction::Left),
            _ => unreachable!(),
        })
        .collect();

    (grid, directions)
}

#[aoc(day15, part2)]
pub fn part2(input: &(Grid<Tile2>, Vec<Direction>)) -> u64 {
    let mut grid = input.0.clone();
    for direction in input.1.iter().copied() {
        if matches!(direction, Direction::Left | Direction::Right) {
            let col_offset: isize = <(isize, isize)>::from(direction).1;
            let mut search_pos = grid.robot_position;
            search_pos.1 += col_offset;
            let next_robot_pos = search_pos;
            let can_move = loop {
                match grid[search_pos] {
                    Tile2::Empty => break true,
                    Tile2::BoxLeft | Tile2::BoxRight => {
                        search_pos.1 += 2 * col_offset;
                    }
                    Tile2::Wall => break false,
                    Tile2::Robot => unreachable!(),
                }
            };
            if can_move {
                let base = grid.width * grid.robot_position.0 as usize;
                let (range, dest) = if direction == Direction::Left {
                    (
                        base + search_pos.1 as usize + 1..=grid.robot_position.1 as usize + base,
                        base + search_pos.1 as usize,
                    )
                } else {
                    (
                        base + grid.robot_position.1 as usize..=base + search_pos.1 as usize - 1,
                        base + grid.robot_position.1 as usize + 1,
                    )
                };
                grid.contents.copy_within(range, dest);
                let curr_pos = grid.robot_position;
                grid[curr_pos] = Tile2::Empty;
                grid.robot_position = next_robot_pos;
            }
        } else {
            let row_offset: isize = <(isize, isize)>::from(direction).0;
            let mut search_pos = grid.robot_position;
            search_pos.0 += row_offset;
            let next_robot_pos = search_pos;
            if can_push(&grid, search_pos, row_offset) {
                do_push(&mut grid, search_pos, row_offset);
                grid[next_robot_pos] = Tile2::Robot;
                let curr_pos = grid.robot_position;
                grid[curr_pos] = Tile2::Empty;
                grid.robot_position = next_robot_pos;
            }
        }
    }

    let mut sum = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid[(row, col)] == Tile2::BoxLeft {
                sum += row as u64 * 100 + col as u64;
            }
        }
    }
    sum
}

fn can_push(grid: &Grid<Tile2>, pos: (isize, isize), row_offset: isize) -> bool {
    match grid[pos] {
        Tile2::Wall => false,
        Tile2::Empty => true,
        Tile2::BoxLeft => {
            can_push(grid, (pos.0 + row_offset, pos.1), row_offset)
                && can_push(grid, (pos.0 + row_offset, pos.1 + 1), row_offset)
        }
        Tile2::BoxRight => {
            can_push(grid, (pos.0 + row_offset, pos.1), row_offset)
                && can_push(grid, (pos.0 + row_offset, pos.1 - 1), row_offset)
        }
        Tile2::Robot => unreachable!(),
    }
}

fn do_push(grid: &mut Grid<Tile2>, pos: (isize, isize), row_offset: isize) {
    match grid[pos] {
        Tile2::Empty => {}
        Tile2::BoxLeft => {
            do_push(grid, (pos.0 + row_offset, pos.1), row_offset);
            do_push(grid, (pos.0 + row_offset, pos.1 + 1), row_offset);
            grid[(pos.0 + row_offset, pos.1)] = Tile2::BoxLeft;
            grid[(pos.0 + row_offset, pos.1 + 1)] = Tile2::BoxRight;
            grid[pos] = Tile2::Empty;
            grid[(pos.0, pos.1 + 1)] = Tile2::Empty;
        }
        Tile2::BoxRight => {
            do_push(grid, (pos.0 + row_offset, pos.1), row_offset);
            do_push(grid, (pos.0 + row_offset, pos.1 - 1), row_offset);
            grid[(pos.0 + row_offset, pos.1)] = Tile2::BoxRight;
            grid[(pos.0 + row_offset, pos.1 - 1)] = Tile2::BoxLeft;
            grid[pos] = Tile2::Empty;
            grid[(pos.0, pos.1 - 1)] = Tile2::Empty;
        }
        _ => unreachable!(),
    }
}
