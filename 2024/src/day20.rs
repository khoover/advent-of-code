use super::*;

use rayon::prelude::*;
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
            let Some(new_pos) = new_pos_row.zip(new_pos_col) else {
                continue;
            };
            if new_pos != last_pos && bytes[new_pos.0 * stride + new_pos.1] != b'#' {
                path.push(new_pos);
                last_pos = pos;
                pos = new_pos;
                continue 'outer;
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
    for (i, pos) in path.into_iter().enumerate() {
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
            let Some(new_pos) = new_pos_row.zip(new_pos_col) else {
                continue;
            };
            if new_pos != last_pos && bytes[new_pos.0 * stride + new_pos.1] != b'#' {
                path.push(new_pos);
                last_pos = pos;
                pos = new_pos;
                continue 'outer;
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

    path.into_par_iter()
        .enumerate()
        .map(|(i, pos)| {
            let mut count = 0;
            let min_offset = -(pos.0.min(20) as isize);
            let max_offset = (rows - pos.0).min(20) as isize;
            for x_offset in min_offset..=max_offset {
                let abs_offset = x_offset.unsigned_abs();
                let new_pos_row = unsafe { pos.0.checked_add_signed(x_offset).unwrap_unchecked() };
                let y_allowable = 20 - abs_offset;
                let min_offset = -(pos.1.min(y_allowable) as isize);
                let max_offset = (columns - pos.1).min(y_allowable) as isize;
                for y_offset in min_offset..=max_offset {
                    let new_pos_col =
                        unsafe { pos.1.checked_add_signed(y_offset).unwrap_unchecked() };
                    count += distance_map
                        .get(&(new_pos_row, new_pos_col))
                        .copied()
                        .is_some_and(|distance| {
                            i + 100 + abs_offset + y_offset.unsigned_abs() <= distance
                        }) as usize;
                }
            }
            count
        })
        .sum()
}
