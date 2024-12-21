use super::*;
use arrayvec::ArrayVec;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

fn ten_key_to_coords(byte: u8) -> (i8, i8) {
    match byte {
        b'7' => (3, 0),
        b'8' => (3, 1),
        b'9' => (3, 2),
        b'4' => (2, 0),
        b'5' => (2, 1),
        b'6' => (2, 2),
        b'1' => (1, 0),
        b'2' => (1, 1),
        b'3' => (1, 2),
        b'0' => (0, 1),
        b'A' => (0, 2),
        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum DirectionPad {
    Left,
    Right,
    Up,
    Down,
    A,
}

impl<T> Index<DirectionPad> for [T; 5] {
    type Output = T;

    fn index(&self, index: DirectionPad) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<DirectionPad> for [T; 5] {
    fn index_mut(&mut self, index: DirectionPad) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Debug for DirectionPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::A => "A",
            Self::Down => "v",
            Self::Left => "<",
            Self::Right => ">",
            Self::Up => "^",
        })
    }
}

impl DirectionPad {
    fn to_coord(self) -> (i8, i8) {
        match self {
            DirectionPad::Up => (1, 1),
            DirectionPad::A => (1, 2),
            DirectionPad::Left => (0, 0),
            DirectionPad::Down => (0, 1),
            DirectionPad::Right => (0, 2),
        }
    }
}

#[aoc(day21, part1)]
pub fn part1(s: &str) -> u64 {
    let mut cache = CacheType::default();
    s.lines().map(|line| part1_per_line(line, &mut cache)).sum()
}

fn part1_per_line(line: &str, cache: &mut CacheType) -> u64 {
    let bytes = line.as_bytes();
    unsafe {
        std::hint::assert_unchecked(bytes.len() == 4);
    }
    let numeric_part =
        (bytes[0] - b'0') as u64 * 100 + (bytes[1] - b'0') as u64 * 10 + (bytes[2] - b'0') as u64;
    let sequence_len: u64 = bytes
        .windows(2)
        .map(|window| cheapest_10key_path(window[0], window[1], 2, cache))
        .sum();
    (sequence_len + cheapest_10key_path(b'A', bytes[0], 2, cache)) * numeric_part
}

#[aoc(day21, part2)]
pub fn part2(s: &str) -> u64 {
    let mut cache = CacheType::default();
    s.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            unsafe {
                std::hint::assert_unchecked(bytes.len() == 4);
            }
            let numeric_part = (bytes[0] - b'0') as u64 * 100
                + (bytes[1] - b'0') as u64 * 10
                + (bytes[2] - b'0') as u64;
            let sequence_len: u64 = bytes
                .windows(2)
                .map(|window| cheapest_10key_path(window[0], window[1], 25, &mut cache))
                .sum();
            (sequence_len + cheapest_10key_path(b'A', bytes[0], 25, &mut cache)) * numeric_part
        })
        .sum()
}

type CacheType = [[[Option<u64>; 26]; 5]; 5];

// Gets the cheapest cost of moving from start to end on the 10-key
fn cheapest_10key_path(start: u8, end: u8, total_layers: u8, cache: &mut CacheType) -> u64 {
    let start_coord = ten_key_to_coords(start);
    let end_coord = ten_key_to_coords(end);
    let mut bfs = VecDeque::new();
    let mut inital_dir = ArrayVec::<DirectionPad, 7>::new();
    inital_dir.push(DirectionPad::A);
    bfs.push_back((start_coord, inital_dir));
    let mut res_10key = u64::MAX;
    while let Some((coord, mut directions)) = bfs.pop_front() {
        if coord == end_coord {
            unsafe {
                directions.push_unchecked(DirectionPad::A);
            }
            res_10key = res_10key.min(
                directions
                    .windows(2)
                    .map(|window| cheapest_dirpad_path(window[0], window[1], total_layers, cache))
                    .sum(),
            );
            continue;
        }
        if coord == (0, 0) {
            continue;
        }

        match coord.0.cmp(&end_coord.0) {
            Ordering::Less => {
                let mut dir_clone = directions.clone();
                unsafe {
                    dir_clone.push_unchecked(DirectionPad::Up);
                }
                bfs.push_back(((coord.0 + 1, coord.1), dir_clone));
            }
            Ordering::Greater => {
                let mut dir_clone = directions.clone();
                unsafe {
                    dir_clone.push_unchecked(DirectionPad::Down);
                }
                bfs.push_back(((coord.0 - 1, coord.1), dir_clone));
            }
            Ordering::Equal => (),
        }

        match coord.1.cmp(&end_coord.1) {
            Ordering::Less => {
                unsafe { directions.push_unchecked(DirectionPad::Right) };
                bfs.push_back(((coord.0, coord.1 + 1), directions));
            }
            Ordering::Greater => {
                unsafe { directions.push_unchecked(DirectionPad::Left) };
                bfs.push_back(((coord.0, coord.1 - 1), directions));
            }
            Ordering::Equal => (),
        }
    }
    debug!(res_10key);
    res_10key
}

fn cheapest_dirpad_path(
    start: DirectionPad,
    end: DirectionPad,
    layer: u8,
    cache: &mut CacheType,
) -> u64 {
    unsafe {
        std::hint::assert_unchecked(layer <= 25);
    }
    if let Some(cost) = cache[start][end][layer as usize] {
        return cost;
    }
    if layer == 0 {
        return 1;
    }
    let start_coord = start.to_coord();
    let end_coord = end.to_coord();
    let mut bfs = VecDeque::new();
    let mut initial_dir = ArrayVec::<DirectionPad, 5>::new();
    initial_dir.push(DirectionPad::A);
    bfs.push_back((start_coord, initial_dir));
    let mut res = u64::MAX;

    while let Some((coord, mut directions)) = bfs.pop_front() {
        if coord == end_coord {
            unsafe { directions.push_unchecked(DirectionPad::A) };
            res = res.min(
                directions
                    .windows(2)
                    .map(|window| cheapest_dirpad_path(window[0], window[1], layer - 1, cache))
                    .sum(),
            );
            continue;
        }
        if coord == (1, 0) {
            continue;
        }

        match coord.0.cmp(&end_coord.0) {
            Ordering::Less => {
                let mut dir_clone = directions.clone();
                unsafe { dir_clone.push_unchecked(DirectionPad::Up) };
                bfs.push_back(((coord.0 + 1, coord.1), dir_clone));
            }
            Ordering::Greater => {
                let mut dir_clone = directions.clone();
                unsafe { dir_clone.push_unchecked(DirectionPad::Down) };
                bfs.push_back(((coord.0 - 1, coord.1), dir_clone));
            }
            Ordering::Equal => (),
        }

        match coord.1.cmp(&end_coord.1) {
            Ordering::Less => {
                unsafe { directions.push_unchecked(DirectionPad::Right) };
                bfs.push_back(((coord.0, coord.1 + 1), directions));
            }
            Ordering::Greater => {
                unsafe { directions.push_unchecked(DirectionPad::Left) };
                bfs.push_back(((coord.0, coord.1 - 1), directions));
            }
            Ordering::Equal => (),
        }
    }
    cache[start][end][layer as usize] = Some(res);
    debug!(start);
    debug!(end);
    debug!(layer);
    debug!(res);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    static SITE_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SITE_INPUT), 126384);
    }

    #[test]
    fn test_per_line() {
        let mut cache = CacheType::default();
        let lines: Vec<_> = SITE_INPUT.lines().collect();
        assert_eq!(part1_per_line(lines[0], &mut cache), 68 * 29, "line 0");
        assert_eq!(part1_per_line(lines[1], &mut cache), 60 * 980, "line 1");
        assert_eq!(part1_per_line(lines[2], &mut cache), 68 * 179, "line 2");
        assert_eq!(part1_per_line(lines[3], &mut cache), 64 * 456, "line 3");
        assert_eq!(part1_per_line(lines[4], &mut cache), 64 * 379, "line 4");
    }
}

// vA<^AA>A -> <vA>^A<<vA^>A>AAvA^A -> v<<A>A>^AvA<^A>Av<<AA>A>^A<Av>A^AvA^AA<vA>^A<A>A
// <AAv>A^A -> <<vA>>^AAv<A>A^A<A>A -> v<<AA>A>^AvAA<^A>AAv<A<A>>^AvA^A<A>Av<<A>>^AvA^A
