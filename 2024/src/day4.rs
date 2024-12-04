use std::cell::Cell;

use nom::AsBytes;

use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn checked_add(self, idx: (usize, usize)) -> Option<(usize, usize)> {
        let pair: (isize, isize) = self.into();
        idx.0
            .checked_add_signed(pair.0)
            .and_then(|x| idx.1.checked_add_signed(pair.1).map(|y| (x, y)))
    }
}

const ALL_DIRS: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::UpRight,
    Direction::UpLeft,
    Direction::DownLeft,
    Direction::DownRight,
];

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    fn check_xmas(idx: (usize, usize), letter_grid: &Vec<Vec<Letter>>) -> usize {
        if letter_grid[idx.0][idx.1] != Letter::X {
            return 0;
        }
        ALL_DIRS
            .into_iter()
            .map(Into::into)
            .filter(|&(row_delta, col_delta)| {
                let mut curr_letter = Letter::X;
                let (mut row, mut col) = idx;
                for _ in 0..3 {
                    if row
                        .checked_add_signed(row_delta)
                        .map(|v| {
                            row = v;
                        })
                        .is_none()
                        || col
                            .checked_add_signed(col_delta)
                            .map(|v| {
                                col = v;
                            })
                            .is_none()
                    {
                        return false;
                    }
                    let Some(next_letter) = letter_grid
                        .get(row)
                        .and_then(|r| r.get(col))
                        .copied()
                        .filter(|next| curr_letter < *next)
                    else {
                        return false;
                    };
                    curr_letter = next_letter;
                }
                true
            })
            .count()
    }

    fn check_x_mas(idx: (usize, usize), letter_grid: &Vec<Vec<Letter>>) -> bool {
        if letter_grid[idx.0][idx.1] != Letter::A {
            return false;
        }

        let Some(up_left) = Direction::UpLeft
            .checked_add(idx)
            .and_then(|(x, y)| letter_grid.get(x).and_then(|r| r.get(y)))
            .copied()
        else {
            return false;
        };
        let Some(up_right) = Direction::UpRight
            .checked_add(idx)
            .and_then(|(x, y)| letter_grid.get(x).and_then(|r| r.get(y)))
            .copied()
        else {
            return false;
        };
        let Some(down_left) = Direction::DownLeft
            .checked_add(idx)
            .and_then(|(x, y)| letter_grid.get(x).and_then(|r| r.get(y)))
            .copied()
        else {
            return false;
        };
        let Some(down_right) = Direction::DownRight
            .checked_add(idx)
            .and_then(|(x, y)| letter_grid.get(x).and_then(|r| r.get(y)))
            .copied()
        else {
            return false;
        };

        matches!(
            (up_left, down_right),
            (Letter::M, Letter::S) | (Letter::S, Letter::M)
        ) && matches!(
            (up_right, down_left),
            (Letter::M, Letter::S) | (Letter::S, Letter::M)
        )
    }

    fn from_byte(b: u8) -> Self {
        match b {
            b'X' => Self::X,
            b'M' => Self::M,
            b'A' => Self::A,
            b'S' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[aoc(day4, part1, Naive)]
fn part1_naive(s: &str) -> usize {
    let mut x_locs: Vec<(usize, usize)> = Vec::new();
    let letter_grid: Vec<Vec<Letter>> = s
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(|(col, b)| {
                    let letter = Letter::from_byte(b);
                    if matches!(letter, Letter::X) {
                        x_locs.push((row, col));
                    }
                    letter
                })
                .collect()
        })
        .collect();
    x_locs
        .into_iter()
        .map(|x_loc| Letter::check_xmas(x_loc, &letter_grid))
        .sum()
}

#[aoc(day4, part2, Naive)]
fn part2_naive(s: &str) -> usize {
    let mut a_locs: Vec<(usize, usize)> = Vec::new();
    let letter_grid: Vec<Vec<Letter>> = s
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(|(col, b)| {
                    let letter = Letter::from_byte(b);
                    if matches!(letter, Letter::A) {
                        a_locs.push((row, col));
                    }
                    letter
                })
                .collect()
        })
        .collect();

    a_locs
        .into_iter()
        .filter(|a_loc| Letter::check_x_mas(*a_loc, &letter_grid))
        .count()
}

#[aoc(day4, part1, Opt)]
fn part1_opt(s: &str) -> usize {
    let Some(columns) = s.find("\n") else {
        return 0;
    };
    let mut x_locs = Vec::new();
    let rows: Cell<usize> = Cell::new(0);
    let col: Cell<usize> = Cell::new(0);
    let letters: Vec<u8> = s
        .as_bytes()
        .iter()
        .copied()
        .filter(|&b| {
            if b == b'\n' {
                rows.set(rows.get() + 1);
                col.set(0);
                false
            } else {
                true
            }
        })
        .map(|b| {
            let curr_col = col.get();
            let res = match b {
                b'X' => {
                    x_locs.push((rows.get(), curr_col));
                    0
                }
                b'M' => 1,
                b'A' => 2,
                b'S' => 3,
                _ => unreachable!(),
            };
            col.set(curr_col + 1);
            res
        })
        .collect();
    let dims = (rows.get(), columns);

    x_locs
        .into_iter()
        //.inspect(|x_loc| println!("{x_loc:?}"))
        .map(|x_loc| check_x_loc(x_loc, dims, letters.as_bytes()))
        //.inspect(|v| println!("{v}"))
        .sum()
}

macro_rules! add_bools {
    ($x:expr) => ($x as usize);
    ($x:expr, $($y:expr),+) => (
        (($x as usize) + add_bools!($($y),+))
    )
}

fn check_x_loc(x_idx: (usize, usize), dims: (usize, usize), letters: &[u8]) -> usize {
    let columns = dims.1;
    unsafe {
        if x_idx.0 < 3 {
            let base = check_xmas_unchecked(x_idx, (1, 0), columns, letters) as usize;
            if x_idx.1 < 3 {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, 1), columns, letters)
                )
            } else if x_idx.1 >= columns - 3 {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, -1), columns, letters)
                )
            } else {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (0, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, -1), columns, letters)
                )
            }
        } else if x_idx.0 + 3 >= dims.0 {
            let base = check_xmas_unchecked(x_idx, (-1, 0), columns, letters) as usize;
            if x_idx.1 < 3 {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, 1), columns, letters)
                )
            } else if x_idx.1 + 3 >= columns {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, -1), columns, letters)
                )
            } else {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (0, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, -1), columns, letters)
                )
            }
        } else {
            let base = add_bools!(
                check_xmas_unchecked(x_idx, (1, 0), columns, letters),
                check_xmas_unchecked(x_idx, (-1, 0), columns, letters)
            );
            if x_idx.1 < 3 {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, 1), columns, letters)
                )
            } else if x_idx.1 + 3 >= columns {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, -1), columns, letters)
                )
            } else {
                base + add_bools!(
                    check_xmas_unchecked(x_idx, (0, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (0, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (-1, -1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, 1), columns, letters),
                    check_xmas_unchecked(x_idx, (1, -1), columns, letters)
                )
            }
        }
    }
}

#[aoc(day4, part2, Opt)]
fn part2_opt(s: &str) -> usize {
    todo!()
}

#[inline(always)]
fn compute_offset_checked(
    idx: (usize, usize),
    offset: (isize, isize),
    dims: (usize, usize),
) -> Option<(usize, usize)> {
    idx.0
        .checked_add_signed(offset.0)
        .filter(|&x| x < dims.0)
        .and_then(|x| {
            idx.1
                .checked_add_signed(offset.1)
                .filter(|&y| y < dims.1)
                .map(|y| (x, y))
        })
}

/// # Safety
/// x_idx + offset * 3 must be within the bounds of the letters grid
pub unsafe fn check_xmas_unchecked(
    x_idx: (usize, usize),
    offset: (isize, isize),
    columns: usize,
    letters: &[u8],
) -> bool {
    let mut index = unsafe { x_idx.0.unchecked_mul(columns).unchecked_add(x_idx.1) };
    let delta = unsafe {
        offset
            .0
            .unchecked_mul(columns as isize)
            .unchecked_add(offset.1)
    };
    (1..=3)
        .map(|mult| {
            // SAFETY: x_idx + offset * 3 is within the bounds of the grid
            (mult
                == unsafe {
                    index = index.wrapping_add_signed(delta);
                    *letters.get_unchecked(index)
                }
                .into()) as u8
        })
        .sum::<u8>()
        == 3
    // (1..=3).all(|mult| {
    //     mult == unsafe {
    //         index = index.wrapping_add_signed(delta);
    //         *letters.get_unchecked(index)
    //     }
    //     .into()
    // })
}

/// # Safety
/// offset.0 * mult and offset.1 * mult must not wrap.
#[inline(always)]
unsafe fn compute_offset_unchecked(
    idx: (usize, usize),
    offset: (isize, isize),
    mult: isize,
) -> (usize, usize) {
    (
        idx.0
            .wrapping_add_signed(unsafe { offset.0.unchecked_mul(mult) }),
        idx.1
            .wrapping_add_signed(unsafe { offset.1.unchecked_mul(mult) }),
    )
}

/// # Safety
/// idx must be a coordinate inside the 2D grid arr represents.
#[inline(always)]
unsafe fn index(idx: (usize, usize), columns: usize, arr: &[u8]) -> u8 {
    unsafe { *arr.get_unchecked(idx.0.unchecked_mul(columns).unchecked_add(idx.1)) }
}

const DIR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part1(s: &str) -> usize {
    part1_naive(s)
}

pub fn part2(s: &str) -> usize {
    part2_naive(s)
}

#[cfg(test)]
mod test {
    use super::*;

    static SITE_INPUT: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    const SITE_PART1_EXPECTED: usize = 18;
    const SITE_PART2_EXPECTED: usize = 9;

    static MY_INPUT: &'static str = include_str!("../input/2024/day4.txt");
    const MY_PART1_EXPECTED: usize = 2583;
    const MY_PART2_EXPECTED: usize = 9;

    #[test]
    fn test_part1_naive_site() {
        assert_eq!(part1_naive(SITE_INPUT), SITE_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_naive_site() {
        assert_eq!(part2_naive(SITE_INPUT), SITE_PART2_EXPECTED);
    }

    #[test]
    fn test_part1_naive_mine() {
        assert_eq!(part1_naive(MY_INPUT), MY_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_naive_mine() {
        assert_eq!(part2_naive(MY_INPUT), MY_PART2_EXPECTED);
    }

    #[test]
    fn test_part1_opt_site() {
        assert_eq!(part1_opt(SITE_INPUT), SITE_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_opt_site() {
        assert_eq!(part2_opt(SITE_INPUT), SITE_PART2_EXPECTED);
    }

    #[test]
    fn test_part1_opt_mine() {
        assert_eq!(part1_opt(MY_INPUT), MY_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_opt_mine() {
        assert_eq!(part2_opt(MY_INPUT), MY_PART2_EXPECTED);
    }
}
