use super::*;
use rustc_hash::FxHashSet;

type ParsedInput = ([Vec<(isize, isize)>; 62], (isize, isize));

#[aoc_generator(day8)]
fn build_map(s: &str) -> ParsedInput {
    let columns = s.find("\n").unwrap();
    let stride = columns + 1;
    let rows = (s.len() + 1) / stride;
    let mut output = [const { Vec::new() }; 62];
    let bytes = s.as_bytes();

    for row in 0..rows {
        let row_bytes = &bytes[row * stride..];
        for (col, b) in row_bytes[..columns].iter().copied().enumerate() {
            let idx = match b {
                b'0'..=b'9' => (b - b'0') as usize,
                b'a'..=b'z' => (b - b'a' + 10) as usize,
                b'A'..=b'Z' => (b - b'A' + 36) as usize,
                _ => continue,
            };
            output[idx].push((row as isize, col as isize));
        }
    }

    (output, (rows as isize, columns as isize))
}

#[aoc(day8, part1)]
fn part1(input: &ParsedInput) -> usize {
    let mut positions = FxHashSet::default();
    let (antennas, (rows, cols)) = input;
    for antenna_collection in antennas.iter() {
        for i in 0..antenna_collection.len() {
            for j in i + 1..antenna_collection.len() {
                let diff_row = antenna_collection[j].0 - antenna_collection[i].0;
                let diff_col = antenna_collection[j].1 - antenna_collection[i].1;
                positions.insert((
                    antenna_collection[j].0 + diff_row,
                    antenna_collection[j].1 + diff_col,
                ));
                positions.insert((
                    antenna_collection[i].0 - diff_row,
                    antenna_collection[i].1 - diff_col,
                ));
            }
        }
    }

    positions
        .into_iter()
        .filter(|(row, col)| (0..*rows).contains(row) && (0..*cols).contains(col))
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &ParsedInput) -> usize {
    let mut positions = FxHashSet::default();
    let (antennas, (rows, cols)) = input;
    let max_repetitions: isize = *rows.min(cols);
    for antenna_collection in antennas.iter() {
        for i in 0..antenna_collection.len() {
            for j in i + 1..antenna_collection.len() {
                let diff_row = antenna_collection[j].0 - antenna_collection[i].0;
                let diff_col = antenna_collection[j].1 - antenna_collection[i].1;
                let mut forward = antenna_collection[j];
                let mut backward = antenna_collection[i];
                for _ in 0..(max_repetitions / diff_row.abs().max(diff_col.abs())) {
                    positions.insert(forward);
                    positions.insert(backward);
                    forward = (forward.0 + diff_row, forward.1 + diff_col);
                    backward = (backward.0 - diff_row, backward.1 - diff_col);
                }
            }
        }
    }

    positions
        .into_iter()
        .filter(|(row, col)| (0..*rows).contains(row) && (0..*cols).contains(col))
        .count()
}
