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

#[aoc(day4, part1)]
pub fn part1(s: &str) -> usize {
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

#[aoc(day4, part2)]
pub fn part2(s: &str) -> usize {
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(SITE_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SITE_INPUT), 9);
    }
}
