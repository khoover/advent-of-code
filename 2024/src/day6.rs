use rustc_hash::FxHashSet;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[aoc(day6, part1)]
fn part1_basic(s: &str) -> usize {
    let mut obstacles = FxHashSet::default();
    let mut guard_start = None;

    let rows = s.lines().count();
    let cols = s.find("\n").unwrap();

    for (row, line) in s.lines().enumerate() {
        for (col, b) in line.as_bytes().iter().copied().enumerate() {
            match b {
                b'#' => {
                    obstacles.insert((row, col));
                }
                b'^' => {
                    guard_start = Some((row, col));
                }
                _ => (),
            }
        }
    }

    let mut visited_positions = FxHashSet::default();
    let mut guard_pos = guard_start.unwrap();
    let mut guard_dir = Direction::Up;
    let mut offset = <(isize, isize)>::from(guard_dir);
    while guard_pos.0 < rows && guard_pos.1 < cols {
        visited_positions.insert(guard_pos);
        let next_pos = (
            guard_pos.0.wrapping_add_signed(offset.0),
            guard_pos.1.wrapping_add_signed(offset.1),
        );
        if obstacles.contains(&next_pos) {
            guard_dir = guard_dir.turn_right();
            offset = guard_dir.into();
        } else {
            guard_pos = next_pos;
        }
    }

    visited_positions.len()
}

#[aoc(day6, part2)]
fn part2_basic(s: &str) -> usize {
    let mut obstacles = FxHashSet::default();
    let mut guard_start = None;

    let rows = s.lines().count();
    let cols = s.find("\n").unwrap();

    for (row, line) in s.lines().enumerate() {
        for (col, b) in line.as_bytes().iter().copied().enumerate() {
            match b {
                b'#' => {
                    obstacles.insert((row, col));
                }
                b'^' => {
                    guard_start = Some((row, col));
                }
                _ => (),
            }
        }
    }

    let guard_start = guard_start.unwrap();
    let mut potential_obstacles = 0;
    let mut visited_positions = FxHashSet::default();
    for new_obstacle_i in 0..rows {
        for new_obstacle_j in 0..cols {
            if (new_obstacle_i == guard_start.0 && new_obstacle_j == guard_start.1)
                || !obstacles.insert((new_obstacle_i, new_obstacle_j))
            {
                continue;
            }
            visited_positions.clear();
            let mut guard_pos = guard_start;
            let mut guard_dir = Direction::Up;
            let mut offset = <(isize, isize)>::from(guard_dir);
            let mut looped = false;
            while guard_pos.0 < rows && guard_pos.1 < cols {
                if !visited_positions.insert((guard_pos, guard_dir)) {
                    looped = true;
                    break;
                }
                let next_pos = (
                    guard_pos.0.wrapping_add_signed(offset.0),
                    guard_pos.1.wrapping_add_signed(offset.1),
                );
                if obstacles.contains(&next_pos) {
                    guard_dir = guard_dir.turn_right();
                    offset = guard_dir.into();
                } else {
                    guard_pos = next_pos;
                }
            }
            obstacles.remove(&(new_obstacle_i, new_obstacle_j));
            potential_obstacles += looped as usize;
        }
    }
    potential_obstacles
}

#[cfg(test)]
mod test {
    use super::*;

    static SITE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    const SITE_PART1_ANSWER: usize = 41;
    const SITE_PART2_ANSWER: usize = 6;

    #[test]
    fn test_part1_basic() {
        assert_eq!(part1_basic(SITE_INPUT), SITE_PART1_ANSWER);
    }

    #[test]
    fn test_part2_basic() {
        assert_eq!(part2_basic(SITE_INPUT), SITE_PART2_ANSWER);
    }
}
