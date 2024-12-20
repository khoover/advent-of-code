use super::*;

use rustc_hash::FxHashMap;

#[aoc(day20, part1)]
pub fn part1(s: &str) -> usize {
    let columns = s.find("\n").unwrap();
    let stride = columns + 1;
    let rows = (s.len() + 1) / stride;
    let start = s.find("S").unwrap();
    let start = (start / stride, start % stride);
    let mut path = Vec::new();
    path.push(start);
    let mut pos = start;
    let mut last_pos = start;
    let bytes = s.as_bytes();
    'outer: while bytes[pos.0 * stride + pos.1] != b'E' {
        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos_row = pos.0.checked_add_signed(offset.0).filter(|x| *x < rows);
            let new_pos_col = pos.1.checked_add_signed(offset.1).filter(|y| *y < columns);
            if let Some(new_pos) = new_pos_row.zip(new_pos_col) {
                if new_pos != last_pos && bytes[new_pos.0 * stride + new_pos.1] != b'#' {
                    path.push(new_pos);
                    last_pos = pos;
                    pos = new_pos;
                    continue 'outer;
                }
            }
        }
        panic!("Uh oh");
    }

    let distance_map = path
        .iter()
        .copied()
        .enumerate()
        .map(|(b, a)| (a, b))
        .collect::<FxHashMap<_, _>>();

    let mut count = 0;
    for (i, pos) in path.iter().copied().enumerate() {
        for offset in [
            (0, 2),
            (0, -2),
            (2, 0),
            (-2, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            let new_pos_row = pos.0.checked_add_signed(offset.0);
            let new_pos_col = pos.1.checked_add_signed(offset.1);
            if let Some(new_pos) = new_pos_row.zip(new_pos_col) {
                if let Some(distance) = distance_map.get(&new_pos) {
                    if i + 102 <= *distance {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[aoc(day20, part2)]
pub fn part2(s: &str) -> usize {
    let columns = s.find("\n").unwrap();
    let stride = columns + 1;
    let rows = (s.len() + 1) / stride;
    let start = s.find("S").unwrap();
    let start = (start / stride, start % stride);
    let mut path = Vec::new();
    path.push(start);
    let mut pos = start;
    let mut last_pos = start;
    let bytes = s.as_bytes();
    'outer: while bytes[pos.0 * stride + pos.1] != b'E' {
        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos_row = pos.0.checked_add_signed(offset.0).filter(|x| *x < rows);
            let new_pos_col = pos.1.checked_add_signed(offset.1).filter(|y| *y < columns);
            if let Some(new_pos) = new_pos_row.zip(new_pos_col) {
                if new_pos != last_pos && bytes[new_pos.0 * stride + new_pos.1] != b'#' {
                    path.push(new_pos);
                    last_pos = pos;
                    pos = new_pos;
                    continue 'outer;
                }
            }
        }
        panic!("Uh oh");
    }

    let distance_map = path
        .iter()
        .copied()
        .enumerate()
        .map(|(b, a)| (a, b))
        .collect::<FxHashMap<_, _>>();

    let mut count = 0;
    for (i, pos) in path.iter().copied().enumerate() {
        for x_offset in -20_isize..=20 {
            let y_allowable = 20 - x_offset.abs();
            for y_offset in -y_allowable..=y_allowable {
                let new_pos_row = pos.0.checked_add_signed(x_offset);
                let new_pos_col = pos.1.checked_add_signed(y_offset);
                if let Some(new_pos) = new_pos_row.zip(new_pos_col) {
                    if let Some(distance) = distance_map.get(&new_pos) {
                        if i + 100 + x_offset.unsigned_abs() + y_offset.unsigned_abs() <= *distance
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
