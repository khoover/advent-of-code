use super::*;
use arrayvec::ArrayVec;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

const fn ten_key_to_coords(byte: u8) -> (i8, i8) {
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

const fn ten_key_to_index(byte: u8) -> usize {
    match byte {
        b'0'..=b'9' => (byte - b'0') as usize,
        b'A' => 10,
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

#[aoc(day21, part1, Pregen)]
pub fn part1_pregen(s: &str) -> u64 {
    const PREGEN: [[u64; 5]; 5] = [
        [1, 5, 7, 4, 8],
        [9, 1, 9, 8, 4],
        [9, 7, 1, 6, 4],
        [8, 4, 4, 1, 7],
        [10, 6, 8, 9, 1],
    ];
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
                .map(|window| cheapest_10key_path_pregen(window[0], window[1], &PREGEN))
                .sum();
            (sequence_len + cheapest_10key_path_pregen(b'A', bytes[0], &PREGEN)) * numeric_part
        })
        .sum()
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

#[aoc(day21, part2, Pregen)]
pub fn part2_pregen(s: &str) -> u64 {
    const PREGEN: [[u64; 5]; 5] = [
        [1, 5743602247, 10218188221, 5743602246, 10218188222],
        [9009012839, 1, 11317884431, 9009012838, 5930403600],
        [12192864309, 9156556999, 1, 8357534516, 5743602246],
        [9009012838, 5743602246, 5930403600, 1, 9686334009],
        [12192864310, 8357534516, 9009012838, 11104086645, 1],
    ];
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
                .map(|window| cheapest_10key_path_pregen(window[0], window[1], &PREGEN))
                .sum();
            (sequence_len + cheapest_10key_path_pregen(b'A', bytes[0], &PREGEN)) * numeric_part
        })
        .sum()
}

#[aoc(day21, part1, UltraPreGen)]
pub fn part1_ultra(s: &str) -> u64 {
    const PREGEN: [[u64; 11]; 11] = [
        [1, 25, 12, 19, 26, 13, 20, 27, 14, 21, 10],
        [21, 1, 10, 11, 12, 19, 20, 13, 20, 21, 22],
        [16, 18, 1, 10, 21, 12, 19, 22, 13, 20, 17],
        [21, 19, 18, 1, 22, 21, 12, 23, 22, 13, 16],
        [22, 16, 17, 18, 1, 10, 11, 12, 19, 20, 23],
        [17, 21, 16, 17, 18, 1, 10, 21, 12, 19, 18],
        [22, 22, 21, 16, 19, 18, 1, 22, 21, 12, 17],
        [23, 17, 18, 19, 16, 17, 18, 1, 10, 11, 24],
        [18, 22, 17, 18, 21, 16, 17, 18, 1, 10, 19],
        [23, 23, 22, 17, 22, 21, 16, 19, 18, 1, 18],
        [18, 26, 21, 12, 27, 22, 13, 28, 23, 14, 1],
    ];
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
                .map(|window| PREGEN[ten_key_to_index(window[0])][ten_key_to_index(window[1])])
                .sum();
            (sequence_len + PREGEN[ten_key_to_index(b'A')][ten_key_to_index(bytes[0])])
                * numeric_part
        })
        .sum()
}

#[aoc(day21, part2, UltraPreGen)]
pub fn part2_ultra(s: &str) -> u64 {
    const PREGEN: [[u64; 11]; 11] = [
        [
            1,
            31420065369,
            14752615084,
            24095973437,
            31420065370,
            14752615085,
            24095973438,
            31420065371,
            14752615086,
            24095973439,
            14287938116,
        ],
        [
            27052881363,
            1,
            14287938116,
            14287938117,
            14752615084,
            24095973437,
            24095973438,
            14752615085,
            24095973438,
            24095973439,
            27052881364,
        ],
        [
            20790420654,
            22411052532,
            1,
            14287938116,
            28154654777,
            14752615084,
            24095973437,
            28154654778,
            14752615085,
            24095973438,
            22778092491,
        ],
        [
            27622800565,
            22411052533,
            22411052532,
            1,
            28154654778,
            28154654777,
            14752615084,
            28154654779,
            28154654778,
            14752615085,
            20790420654,
        ],
        [
            27052881364,
            20790420654,
            22778092491,
            22778092492,
            1,
            14287938116,
            14287938117,
            14752615084,
            24095973437,
            24095973438,
            27052881365,
        ],
        [
            20790420655,
            27622800565,
            20790420654,
            22778092491,
            22411052532,
            1,
            14287938116,
            28154654777,
            14752615084,
            24095973437,
            22778092492,
        ],
        [
            27622800566,
            27622800566,
            27622800565,
            20790420654,
            22411052533,
            22411052532,
            1,
            28154654778,
            28154654777,
            14752615084,
            20790420655,
        ],
        [
            27052881365,
            20790420655,
            22778092492,
            22778092493,
            20790420654,
            22778092491,
            22778092492,
            1,
            14287938116,
            14287938117,
            27052881366,
        ],
        [
            20790420656,
            27622800566,
            20790420655,
            22778092492,
            27622800565,
            20790420654,
            22778092491,
            22411052532,
            1,
            14287938116,
            22778092493,
        ],
        [
            27622800567,
            27622800567,
            27622800566,
            20790420655,
            27622800566,
            27622800565,
            20790420654,
            22411052533,
            22411052532,
            1,
            20790420656,
        ],
        [
            22411052532,
            31420065370,
            28154654777,
            14752615084,
            31420065371,
            28154654778,
            14752615085,
            31420065372,
            28154654779,
            14752615086,
            1,
        ],
    ];
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
                .map(|window| PREGEN[ten_key_to_index(window[0])][ten_key_to_index(window[1])])
                .sum();
            (sequence_len + PREGEN[ten_key_to_index(b'A')][ten_key_to_index(bytes[0])])
                * numeric_part
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

fn cheapest_10key_path_pregen(start: u8, end: u8, pregen_cache: &[[u64; 5]; 5]) -> u64 {
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
                    .map(|window| pregen_cache[window[0]][window[1]])
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
    const ALL_DIRPADS: [DirectionPad; 5] = [
        DirectionPad::A,
        DirectionPad::Down,
        DirectionPad::Up,
        DirectionPad::Left,
        DirectionPad::Right,
    ];

    const ALL_KEYPADS: [u8; 11] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A',
    ];

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

    #[test]
    fn pregen_2_deep() {
        let mut cache = CacheType::default();
        let mut final_result = [[0_u64; 5]; 5];
        for start in ALL_DIRPADS {
            for end in ALL_DIRPADS {
                final_result[start][end] = cheapest_dirpad_path(start, end, 2, &mut cache);
            }
        }
        println!("{final_result:?}");
    }

    #[test]
    fn pregen_25_deep() {
        let mut cache = CacheType::default();
        let mut final_result = [[0_u64; 5]; 5];
        for start in ALL_DIRPADS {
            for end in ALL_DIRPADS {
                final_result[start][end] = cheapest_dirpad_path(start, end, 25, &mut cache);
            }
        }
        println!("{final_result:?}");
    }

    #[test]
    fn pregen_part1_keypad_distance() {
        const PREGEN: [[u64; 5]; 5] = [
            [1, 5, 7, 4, 8],
            [9, 1, 9, 8, 4],
            [9, 7, 1, 6, 4],
            [8, 4, 4, 1, 7],
            [10, 6, 8, 9, 1],
        ];
        let mut final_result = [[0_u64; 11]; 11];
        for start in ALL_KEYPADS {
            for end in ALL_KEYPADS {
                final_result[ten_key_to_index(start)][ten_key_to_index(end)] =
                    cheapest_10key_path_pregen(start, end, &PREGEN);
            }
        }
        println!("{final_result:?}");
    }

    #[test]
    fn pregen_part2_keypad_distance() {
        const PREGEN: [[u64; 5]; 5] = [
            [1, 5743602247, 10218188221, 5743602246, 10218188222],
            [9009012839, 1, 11317884431, 9009012838, 5930403600],
            [12192864309, 9156556999, 1, 8357534516, 5743602246],
            [9009012838, 5743602246, 5930403600, 1, 9686334009],
            [12192864310, 8357534516, 9009012838, 11104086645, 1],
        ];
        let mut final_result = [[0_u64; 11]; 11];
        for start in ALL_KEYPADS {
            for end in ALL_KEYPADS {
                final_result[ten_key_to_index(start)][ten_key_to_index(end)] =
                    cheapest_10key_path_pregen(start, end, &PREGEN);
            }
        }
        println!("{final_result:?}");
    }

    #[test]
    fn curious() {
        const PREGEN: [[u64; 5]; 5] = [
            [1, 5, 7, 4, 8],
            [9, 1, 9, 8, 4],
            [9, 7, 1, 6, 4],
            [8, 4, 4, 1, 7],
            [10, 6, 8, 9, 1],
        ];
        for a in ALL_DIRPADS {
            for b in ALL_DIRPADS {
                if (matches!(a, DirectionPad::Up | DirectionPad::Down)
                    && matches!(b, DirectionPad::Up | DirectionPad::Down))
                    || (matches!(a, DirectionPad::Left | DirectionPad::Right)
                        && matches!(a, DirectionPad::Left | DirectionPad::Right))
                {
                    continue;
                }
                assert_eq!(
                    PREGEN[DirectionPad::A][a] + PREGEN[a][b] + PREGEN[b][DirectionPad::A],
                    PREGEN[DirectionPad::A][b] + PREGEN[b][a] + PREGEN[a][DirectionPad::A],
                    "{:?}{:?}{:?}{:?} has different cost from {:?}{:?}{:?}{:?}",
                    DirectionPad::A,
                    a,
                    b,
                    DirectionPad::A,
                    DirectionPad::A,
                    b,
                    a,
                    DirectionPad::A
                );
            }
        }
    }
}

// vA<^AA>A -> <vA>^A<<vA^>A>AAvA^A -> v<<A>A>^AvA<^A>Av<<AA>A>^A<Av>A^AvA^AA<vA>^A<A>A
// <AAv>A^A -> <<vA>>^AAv<A>A^A<A>A -> v<<AA>A>^AvAA<^A>AAv<A<A>>^AvA^A<A>Av<<A>>^AvA^A
