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
    fn check_xmas(idx: (usize, usize), letter_grid: &[Vec<Letter>]) -> usize {
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

    fn check_x_mas(idx: (usize, usize), letter_grid: &[Vec<Letter>]) -> bool {
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

#[aoc(day4, part1, Simd)]
fn part1_simd(s: &str) -> usize {
    let input = s.as_bytes();
    let Some(columns) = input
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, b)| (b == b'\n').then_some(idx))
        .next()
        .filter(|&idx| idx != 0)
    else {
        return 0;
    };
    let stride = unsafe { columns.unchecked_add(1) };
    let rows = (input.len() + 1) / stride;
    let mut sum = 0;

    // Covers all possible vertical and diagonal combos, plus all horizontal ones
    // in all but the last 3 rows.
    for i1 in (0..rows - 3).map(|x| x * stride) {
        let i2 = i1 + stride;
        let i3 = i1 + 2 * stride;
        let i4 = i1 + 3 * stride;

        for offset in 0..columns - 3 {
            debug!(offset);
            let r1: &[u8; 4] = unsafe {
                input
                    .get_unchecked(i1 + offset..i1 + offset + 4)
                    .try_into()
                    .unwrap_unchecked()
            };
            let r2: &[u8; 4] = unsafe {
                input
                    .get_unchecked(i2 + offset..i2 + offset + 4)
                    .try_into()
                    .unwrap_unchecked()
            };
            let r3: &[u8; 4] = unsafe {
                input
                    .get_unchecked(i3 + offset..i3 + offset + 4)
                    .try_into()
                    .unwrap_unchecked()
            };
            let r4: &[u8; 4] = unsafe {
                input
                    .get_unchecked(i4 + offset..i4 + offset + 4)
                    .try_into()
                    .unwrap_unchecked()
            };
            let col = u32::from_ne_bytes([r1[0], r2[0], r3[0], r4[0]]);
            sum += (col == XMAS || col == XMAS.swap_bytes()) as usize;
            debug!(sum);

            let row = u32::from_ne_bytes(*r1);
            sum += (row == XMAS || row == XMAS.swap_bytes()) as usize;
            debug!(sum);

            let diagonal_1 = u32::from_ne_bytes([r1[0], r2[1], r3[2], r4[3]]);
            debug!(hex diagonal_1);
            sum += (diagonal_1 == XMAS || diagonal_1 == XMAS.swap_bytes()) as usize;
            debug!(sum);

            let diagonal_2 = u32::from_ne_bytes([r1[3], r2[2], r3[1], r4[0]]);
            sum += (diagonal_2 == XMAS || diagonal_2 == XMAS.swap_bytes()) as usize;
            debug!(sum);
        }

        sum += unsafe {
            check_columns_simd(
                input
                    .get_unchecked(i1 + columns - 3..i1 + columns)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(i2 + columns - 3..i2 + columns)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(i3 + columns - 3..i3 + columns)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(i4 + columns - 3..i4 + columns)
                    .try_into()
                    .unwrap_unchecked(),
            )
        };
        debug!(sum);
    }

    // Now handle the horizontal ones in the last 3 rows
    for i in (rows - 3..rows).map(|x| x * stride) {
        debug!(i);
        (0..columns - 3).for_each(|offset| {
            debug!(offset);
            let row = u32::from_ne_bytes(unsafe {
                input
                    .get_unchecked(i + offset..i + offset + 4)
                    .try_into()
                    .unwrap_unchecked()
            });
            sum += (row == XMAS || row == XMAS.swap_bytes()) as usize;
            debug!(sum);
        });
    }

    sum
}

const XMAS: u32 = 0x58_4D_41_53_u32;

fn check_columns_simd(x_row: &[u8; 3], m_row: &[u8; 3], a_row: &[u8; 3], s_row: &[u8; 3]) -> usize {
    x_row
        .iter()
        .copied()
        .zip(m_row.iter().copied())
        .zip(a_row.iter().copied())
        .zip(s_row.iter().copied())
        .filter(|&(((x, m), a), s)| {
            let candidate = u32::from_le_bytes([x, m, a, s]);
            candidate == XMAS || candidate == XMAS.swap_bytes()
        })
        .count()
}

const PART2_FAST: usize = 16;
const A: u8 = b'A';
const M: u8 = b'M';
const S: u8 = b'S';

#[aoc(day4, part2, Simd)]
fn part2_simd(s: &str) -> usize {
    let input = s.as_bytes();
    let Some(columns) = input
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, b)| (b == b'\n').then_some(idx))
        .next()
        .filter(|&idx| idx != 0)
    else {
        return 0;
    };
    let stride = unsafe { columns.unchecked_add(1) };
    let end_bound = input.len() - 2 * stride - 2;
    let mut sum = 0;

    let top_right_offset = 2;
    let a_offset = stride + 1;
    let bottom_left_offset = 2 * stride;
    let bottom_right_offset = 2 * stride + 2;
    let mut top_left_offset = 0;

    while top_left_offset < end_bound - PART2_FAST {
        sum += unsafe {
            part2_fast_check(
                input
                    .get_unchecked(top_left_offset..top_left_offset + PART2_FAST)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_right_offset + top_left_offset
                            ..top_right_offset + top_left_offset + PART2_FAST,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        bottom_left_offset + top_left_offset
                            ..bottom_left_offset + top_left_offset + PART2_FAST,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        bottom_right_offset + top_left_offset
                            ..bottom_right_offset + top_left_offset + PART2_FAST,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        a_offset + top_left_offset..a_offset + top_left_offset + PART2_FAST,
                    )
                    .try_into()
                    .unwrap_unchecked(),
            )
        };
        top_left_offset += PART2_FAST;
    }

    while top_left_offset < end_bound {
        let top_left = unsafe { *input.get_unchecked(top_left_offset) };
        let top_right = unsafe { *input.get_unchecked(top_right_offset + top_left_offset) };
        let bottom_left = unsafe { *input.get_unchecked(bottom_left_offset + top_left_offset) };
        let bottom_right = unsafe { *input.get_unchecked(bottom_right_offset + top_left_offset) };
        let center = unsafe { *input.get_unchecked(a_offset + top_left_offset) };
        sum += ((center == A)
            && ((top_left == M && bottom_right == S) || (top_left == S && bottom_right == M))
            && ((top_right == M && bottom_left == S) || (top_right == S && bottom_left == M)))
            as usize;
        top_left_offset += 1;
    }

    sum
}

#[inline(always)]
fn part2_fast_check(
    top_left: &[u8; PART2_FAST],
    top_right: &[u8; PART2_FAST],
    bottom_left: &[u8; PART2_FAST],
    bottom_right: &[u8; PART2_FAST],
    center_list: &[u8; PART2_FAST],
) -> usize {
    let top_left_ms = arr_eq(top_left, M);
    let bottom_right_ses = arr_eq(bottom_right, S);
    let diag_1_a = arr_and(top_left_ms, bottom_right_ses);

    let top_left_ses = arr_eq(top_left, S);
    let bottom_right_ms = arr_eq(bottom_right, M);
    let diag_1_b = arr_and(top_left_ses, bottom_right_ms);
    let diag_1 = arr_or(diag_1_a, diag_1_b);

    let top_right_ms = arr_eq(top_right, M);
    let bottom_left_ses = arr_eq(bottom_left, S);
    let diag_2_a = arr_and(top_right_ms, bottom_left_ses);

    let top_right_ses = arr_eq(top_right, S);
    let bottom_left_ms = arr_eq(bottom_left, M);
    let diag_2_b = arr_and(top_right_ses, bottom_left_ms);
    let diag_2 = arr_or(diag_2_a, diag_2_b);

    let valid_centers = arr_eq(center_list, A);
    arr_and(valid_centers, arr_and(diag_1, diag_2))
        .into_iter()
        .filter(|&x| x)
        .count()
}

#[inline(always)]
fn arr_eq(arr: &[u8; PART2_FAST], c: u8) -> [bool; PART2_FAST] {
    let mut out = [false; PART2_FAST];
    for i in 0..PART2_FAST {
        out[i] = arr[i] == c;
    }
    out
}

#[inline(always)]
fn arr_and(a: [bool; PART2_FAST], b: [bool; PART2_FAST]) -> [bool; PART2_FAST] {
    let mut out = [false; PART2_FAST];
    for i in 0..PART2_FAST {
        out[i] = a[i] & b[i];
    }
    out
}

#[inline(always)]
fn arr_or(a: [bool; PART2_FAST], b: [bool; PART2_FAST]) -> [bool; PART2_FAST] {
    let mut out = [false; PART2_FAST];
    for i in 0..PART2_FAST {
        out[i] = a[i] | b[i];
    }
    out
}

pub fn part1(s: &str) -> usize {
    part1_simd(s)
}

pub fn part2(s: &str) -> usize {
    part2_simd(s)
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
    const MY_PART2_EXPECTED: usize = 1978;

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
    fn test_part1_simd_site() {
        assert_eq!(part1_simd(SITE_INPUT), SITE_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_simd_site() {
        assert_eq!(part2_simd(SITE_INPUT), SITE_PART2_EXPECTED);
    }

    #[test]
    fn test_part1_simd_mine() {
        assert_eq!(part1_simd(MY_INPUT), MY_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_simd_mine() {
        assert_eq!(part2_simd(MY_INPUT), MY_PART2_EXPECTED);
    }
}
