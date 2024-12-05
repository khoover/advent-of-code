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

const PART1_WIDTH: usize = 32;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_simd(s: &str) -> usize {
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
    let diag_1_offsets = [0, stride + 1, 2 * stride + 2, 3 * stride + 3];
    let diag_2_offsets = [3, stride + 2, 2 * stride + 1, 3 * stride];
    let diag_end = input.len().saturating_sub(diag_1_offsets[3]);
    let fast_iter_end = diag_end.saturating_sub(PART1_WIDTH);
    let fast_row_end = input.len().saturating_sub(PART1_WIDTH + 3);
    let Some(final_index) = input.len().checked_sub(3) else {
        return 0;
    };
    let remaining_columns = input.len().saturating_sub(3 * stride).min(3);
    let mut top_left = 0;
    let mut sum = 0;

    while top_left < fast_iter_end {
        // Top row
        sum += unsafe {
            check_xmas_simd(
                input
                    .get_unchecked(top_left..top_left + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 1..top_left + 1 + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 2..top_left + 2 + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 3..top_left + 3 + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
            )
        };

        // Left column
        sum += unsafe {
            check_xmas_simd(
                input
                    .get_unchecked(top_left..top_left + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + stride..top_left + stride + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 2 * stride..top_left + 2 * stride + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 3 * stride..top_left + 3 * stride + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
            )
        };

        // Diag 1, \
        sum += unsafe {
            check_xmas_simd(
                input
                    .get_unchecked(
                        top_left + diag_1_offsets[0]..top_left + diag_1_offsets[0] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_left + diag_1_offsets[1]..top_left + diag_1_offsets[1] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_left + diag_1_offsets[2]..top_left + diag_1_offsets[2] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_left + diag_1_offsets[3]..top_left + diag_1_offsets[3] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
            )
        };

        // Diag 2, /
        sum += unsafe {
            check_xmas_simd(
                input
                    .get_unchecked(
                        top_left + diag_2_offsets[0]..top_left + diag_2_offsets[0] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_left + diag_2_offsets[1]..top_left + diag_2_offsets[1] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_left + diag_2_offsets[2]..top_left + diag_2_offsets[2] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(
                        top_left + diag_2_offsets[3]..top_left + diag_2_offsets[3] + PART1_WIDTH,
                    )
                    .try_into()
                    .unwrap_unchecked(),
            )
        };
        top_left += PART1_WIDTH;
    }

    // Handle remainder of the original pattern, just send it to the SIMD and let it figure it out.
    if top_left < diag_end {
        // Top row
        sum += unsafe {
            check_remainder_slog(
                input.get_unchecked(top_left..diag_end),
                input.get_unchecked(top_left + 1..diag_end + 1),
                input.get_unchecked(top_left + 2..diag_end + 2),
                input.get_unchecked(top_left + 3..diag_end + 3),
            )
        };

        // Left column
        sum += unsafe {
            check_remainder_slog(
                input.get_unchecked(top_left..diag_end),
                input.get_unchecked(top_left + stride..diag_end + stride),
                input.get_unchecked(top_left + 2 * stride..diag_end + 2 * stride),
                input.get_unchecked(top_left + 3 * stride..diag_end + 3 * stride),
            )
        };

        // Diag 1, \
        sum += unsafe {
            check_remainder_slog(
                input.get_unchecked(top_left + diag_1_offsets[0]..diag_end + diag_1_offsets[0]),
                input.get_unchecked(top_left + diag_1_offsets[1]..diag_end + diag_1_offsets[1]),
                input.get_unchecked(top_left + diag_1_offsets[2]..diag_end + diag_1_offsets[2]),
                input.get_unchecked(top_left + diag_1_offsets[3]..diag_end + diag_1_offsets[3]),
            )
        };

        // Diag 2, /
        sum += unsafe {
            check_remainder_slog(
                input.get_unchecked(top_left + diag_2_offsets[0]..diag_end + diag_2_offsets[0]),
                input.get_unchecked(top_left + diag_2_offsets[1]..diag_end + diag_2_offsets[1]),
                input.get_unchecked(top_left + diag_2_offsets[2]..diag_end + diag_2_offsets[2]),
                input.get_unchecked(top_left + diag_2_offsets[3]..diag_end + diag_2_offsets[3]),
            )
        };

        top_left = diag_end;
    }

    // Now the remaining columns, <= 3 of them
    if remaining_columns > 0 {
        sum += unsafe {
            check_remainder_columns_simd(
                input.get_unchecked(top_left..top_left + remaining_columns),
                input.get_unchecked(top_left + stride..top_left + stride + remaining_columns),
                input.get_unchecked(
                    top_left + 2 * stride..top_left + 2 * stride + remaining_columns,
                ),
                input.get_unchecked(
                    top_left + 3 * stride..top_left + 3 * stride + remaining_columns,
                ),
            )
        };
    }

    // And now blaze through the rows
    while top_left < fast_row_end {
        sum += unsafe {
            check_xmas_simd(
                input
                    .get_unchecked(top_left..top_left + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 1..top_left + 1 + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 2..top_left + 2 + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
                input
                    .get_unchecked(top_left + 3..top_left + 3 + PART1_WIDTH)
                    .try_into()
                    .unwrap_unchecked(),
            )
        };
        top_left += PART1_WIDTH;
    }

    // And get the remainder
    sum + unsafe {
        check_remainder_slog(
            input.get_unchecked(top_left..final_index),
            input.get_unchecked(top_left + 1..final_index + 1),
            input.get_unchecked(top_left + 2..final_index + 2),
            input.get_unchecked(top_left + 3..final_index + 3),
        )
    }
}

const XMAS: u32 = 0x58_4D_41_53_u32;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn check_remainder_slog(x_row: &[u8], m_row: &[u8], a_row: &[u8], s_row: &[u8]) -> usize {
    unsafe {
        std::hint::assert_unchecked(
            x_row.len() < PART1_WIDTH
                && x_row.len() == m_row.len()
                && x_row.len() == a_row.len()
                && x_row.len() == s_row.len(),
        );
    }
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

fn check_remainder_columns_simd(x_row: &[u8], m_row: &[u8], a_row: &[u8], s_row: &[u8]) -> usize {
    unsafe {
        std::hint::assert_unchecked(x_row.len() <= 3);
        check_remainder_slog(x_row, m_row, a_row, s_row)
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn check_xmas_simd(
    xs: &[u8; PART1_WIDTH],
    ma: &[u8; PART1_WIDTH],
    am: &[u8; PART1_WIDTH],
    sx: &[u8; PART1_WIDTH],
) -> usize {
    // Can use OR here since any base will have either forward or back, never both.
    let found = arr_or(
        arr_and(
            arr_and(arr_eq(xs, X), arr_eq(ma, M)),
            arr_and(arr_eq(am, A), arr_eq(sx, S)),
        ),
        arr_and(
            arr_and(arr_eq(xs, S), arr_eq(ma, A)),
            arr_and(arr_eq(am, M), arr_eq(sx, X)),
        ),
    );

    found.into_iter().map(|x| x as usize).sum::<usize>()
}

const PART2_FAST: usize = 16;
const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_simd(s: &str) -> usize {
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
    let Some(end_bound) = input.len().checked_sub(2 * stride + 2) else {
        return 0;
    };
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

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_fast_check(
    top_left: &[u8; PART2_FAST],
    top_right: &[u8; PART2_FAST],
    bottom_left: &[u8; PART2_FAST],
    bottom_right: &[u8; PART2_FAST],
    center_list: &[u8; PART2_FAST],
) -> usize {
    let diag_1_diff = arr_diff(top_left, bottom_right);
    let diag_1 = arr_or(
        arr_eq(&diag_1_diff, S - M),
        arr_eq(&diag_1_diff, M.wrapping_sub(S)),
    );

    let diag_2_diff = arr_diff(top_right, bottom_left);
    let diag_2 = arr_or(
        arr_eq(&diag_2_diff, S - M),
        arr_eq(&diag_2_diff, M.wrapping_sub(S)),
    );

    let valid_centers = arr_eq(center_list, A);
    arr_and(valid_centers, arr_and(diag_1, diag_2))
        .into_iter()
        .filter(|&x| x)
        .count()
}

#[inline(always)]
fn arr_diff<const N: usize>(a: &[u8; N], b: &[u8; N]) -> [u8; N] {
    let mut out = [0; N];
    for i in 0..N {
        out[i] = a[i].wrapping_sub(b[i]);
    }
    out
}

#[aoc(day4, part1, Simd)]
pub fn part1(s: &str) -> usize {
    unsafe { part1_simd(s) }
}

#[aoc(day4, part2, Simd)]
pub fn part2(s: &str) -> usize {
    unsafe { part2_simd(s) }
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
        assert_eq!(unsafe { part1_simd(SITE_INPUT) }, SITE_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_simd_site() {
        assert_eq!(unsafe { part2_simd(SITE_INPUT) }, SITE_PART2_EXPECTED);
    }

    #[test]
    fn test_part1_simd_mine() {
        assert_eq!(unsafe { part1_simd(MY_INPUT) }, MY_PART1_EXPECTED);
    }

    #[test]
    fn test_part2_simd_mine() {
        assert_eq!(unsafe { part2_simd(MY_INPUT) }, MY_PART2_EXPECTED);
    }
}
