use std::cell::Cell;

use super::*;

use arrayvec::ArrayVec;
use petgraph::{
    algo::{astar, dijkstra},
    graphmap::DiGraphMap,
    visit::{depth_first_search, Control, DfsEvent},
    Direction as PetgraphDirection,
};
use rustc_hash::FxHashSet;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const ALL_DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Right,
    Direction::Down,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum EdgeType {
    Straight,
    Turn,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn offset(self, (row, column): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (row - 1, column),
            Direction::Down => (row + 1, column),
            Direction::Left => (row, column - 1),
            Direction::Right => (row, column + 1),
        }
    }
}

#[aoc_generator(day16)]
fn gen(
    s: &str,
) -> (
    DiGraphMap<(usize, usize, Direction), EdgeType>,
    (usize, usize),
    (usize, usize),
) {
    let columns = s.find("\n").unwrap();
    let stride = columns + 1;
    let end_pos = Cell::new(None);
    let start_pos = Cell::new(None);
    let end_pos_ref = &end_pos;
    let start_pos_ref = &start_pos;
    let mut g: DiGraphMap<(usize, usize, Direction), EdgeType> =
        DiGraphMap::from_edges(s.lines().enumerate().flat_map(|(row, line)| {
            line.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, b)| *b != b'#')
                .flat_map(move |(column, byte)| {
                    if byte == b'E' {
                        assert!(end_pos_ref.replace(Some((row, column))).is_none());
                    } else if byte == b'S' {
                        assert!(start_pos_ref.replace(Some((row, column))).is_none());
                    }
                    ALL_DIRS.into_iter().flat_map(move |dir| {
                        let mut edges = ArrayVec::<
                            (
                                (usize, usize, Direction),
                                (usize, usize, Direction),
                                EdgeType,
                            ),
                            3,
                        >::new();
                        edges.push((
                            (row, column, dir),
                            (row, column, dir.turn_left()),
                            EdgeType::Turn,
                        ));
                        edges.push((
                            (row, column, dir),
                            (row, column, dir.turn_right()),
                            EdgeType::Turn,
                        ));
                        let dst = dir.offset((row, column));
                        if s.as_bytes()[dst.0 * stride + dst.1] != b'#' {
                            edges.push((
                                (row, column, dir),
                                (dst.0, dst.1, dir),
                                EdgeType::Straight,
                            ));
                        }
                        edges
                    })
                })
        }));
    (g, start_pos.get().unwrap(), end_pos.get().unwrap())
}

#[aoc(day16, part1)]
pub fn part1(
    (g, start_pos, end): &(
        DiGraphMap<(usize, usize, Direction), EdgeType>,
        (usize, usize),
        (usize, usize),
    ),
) -> u64 {
    let start = (start_pos.0, start_pos.1, Direction::Right);

    astar(
        &g,
        start,
        |(row, col, _)| (row, col) == *end,
        |(_, _, edge_type)| match edge_type {
            EdgeType::Straight => 1_u64,
            EdgeType::Turn => 1000_u64,
        },
        |(row, col, dir)| {
            let row_diff = row as isize - end.0 as isize;
            let col_diff = col as isize - end.1 as isize;
            let base_cost = row_diff.abs() + col_diff.abs();
            base_cost as u64
                + match (row_diff.signum(), col_diff.signum(), dir) {
                    (1, 0, Direction::Down)
                    | (-1, 0, Direction::Up)
                    | (0, 1, Direction::Right)
                    | (0, -1, Direction::Left) => 0,
                    _ => 1000,
                }
        },
    )
    .unwrap()
    .0
}

#[aoc(day16, part2)]
pub fn part2(
    (g, start_pos, end): &(
        DiGraphMap<(usize, usize, Direction), EdgeType>,
        (usize, usize),
        (usize, usize),
    ),
) -> usize {
    let mut seats = FxHashSet::<(usize, usize)>::default();
    let mut reverse_g = DiGraphMap::from_edges(
        g.all_edges()
            .map(|(start, end, edge_type)| (end, start, edge_type)),
    );
    for dir in ALL_DIRS {
        reverse_g.add_edge(
            (0, 0, Direction::Right),
            (end.0, end.1, dir),
            EdgeType::Straight,
        );
    }

    let forward_costs = dijkstra(
        &g,
        (start_pos.0, start_pos.1, Direction::Right),
        None,
        |(_, _, edge_type)| match edge_type {
            EdgeType::Straight => 1_u64,
            EdgeType::Turn => 1000_u64,
        },
    );
    let reverse_costs = dijkstra(
        &reverse_g,
        (0, 0, Direction::Right),
        None,
        |(_, _, edge_type)| match edge_type {
            EdgeType::Straight => 1_u64,
            EdgeType::Turn => 1000_u64,
        },
    );

    const GOAL: u64 = 73404;

    for (node, cost) in forward_costs.into_iter() {
        if reverse_costs.get(&node).unwrap() - 1 + cost == GOAL {
            seats.insert((node.0, node.1));
        }
    }
    seats.len()
}
