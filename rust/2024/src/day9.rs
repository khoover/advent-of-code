use std::{cmp::Reverse, collections::BinaryHeap};

use super::*;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let mut bytes = input.trim_ascii_end().as_bytes();
    let mut block_idx = 0;
    let mut spaces_remaining = 0;
    let mut blocks_remaining = 0;
    let mut left_file_id = 0;
    let mut right_file_id = bytes.len() as u64 / 2 + 1;
    let mut checksum = 0;

    // Invariant: bytes starts with the next file to add on the left, and the next file to add on the right
    while bytes.len() > 2 {
        if spaces_remaining == 0 {
            let left_width = bytes[0] - b'0';
            checksum += unsafe { calc_checksum_increment(left_file_id, left_width, block_idx) };
            block_idx += left_width as u64;
            left_file_id += 1;
            spaces_remaining = bytes[1] - b'0';
            bytes = &bytes[2..];
        } else if blocks_remaining == 0 {
            right_file_id -= 1;
            blocks_remaining = bytes.last().unwrap() - b'0';
            bytes = &bytes[..bytes.len() - 2];
        } else {
            let moved_width = blocks_remaining.min(spaces_remaining);
            checksum += unsafe { calc_checksum_increment(right_file_id, moved_width, block_idx) };
            block_idx += moved_width as u64;
            blocks_remaining -= moved_width;
            spaces_remaining -= moved_width;
        }
    }

    if spaces_remaining != 0 {
        let moved_width = blocks_remaining.min(spaces_remaining);
        checksum += unsafe { calc_checksum_increment(right_file_id, moved_width, block_idx) };
        blocks_remaining -= moved_width;
        block_idx += moved_width.max(spaces_remaining) as u64;
    }
    let left_width = bytes[0] - b'0';
    checksum += unsafe { calc_checksum_increment(left_file_id, left_width, block_idx) };
    block_idx += left_width as u64;
    if blocks_remaining != 0 {
        checksum += unsafe { calc_checksum_increment(right_file_id, blocks_remaining, block_idx) };
    }

    checksum
}

/// SAFETY:
/// width < 10
unsafe fn calc_checksum_increment(file_id: u64, width: u8, block_idx: u64) -> u64 {
    const SLOP_TABLE: [u8; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

    std::hint::assert_unchecked(width < 10);
    file_id * (width as u64 * block_idx + SLOP_TABLE[width as usize] as u64)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct File {
    id: u64,
    width: u8,
    block_idx: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Gap {
    width: u8,
    block_idx: u64,
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.trim_ascii_end().as_bytes();
    let mut block_idx = 0;
    let mut gaps: Vec<Gap> = Vec::with_capacity(bytes.len() / 2);
    let mut files: Vec<File> = Vec::with_capacity(bytes.len() / 2 + 1);
    let mut gap_lookup: [BinaryHeap<Reverse<usize>>; 10] = [const { BinaryHeap::new() }; 10];
    bytes.iter().copied().enumerate().for_each(|(idx, b)| {
        if idx % 2 == 0 {
            let width = b - b'0';
            let res = File {
                id: idx as u64 / 2,
                width,
                block_idx,
            };
            block_idx += width as u64;
            files.push(res);
        } else {
            let width = b - b'0';
            unsafe {
                std::hint::assert_unchecked(width < 10);
            }
            gap_lookup[width as usize].push(Reverse(idx / 2));
            let res = Gap { width, block_idx };
            block_idx += width as u64;
            gaps.push(res);
        }
    });

    let mut checksum = 0;

    for File {
        id,
        width,
        block_idx,
    } in files.into_iter().rev()
    {
        let first_gap = gap_lookup[width as usize..]
            .iter()
            .filter_map(|heap| heap.peek().map(|gap_idx| gap_idx.0))
            .min();
        match first_gap {
            Some(gap_idx) if (gap_idx as u64) < id => {
                let gap = &mut gaps[gap_idx];
                gap_lookup[gap.width as usize].pop();
                checksum += unsafe { calc_checksum_increment(id, width, gap.block_idx) };
                gap.block_idx += width as u64;
                gap.width -= width;
                gap_lookup[gap.width as usize].push(Reverse(gap_idx));
            }
            _ => {
                checksum += unsafe { calc_checksum_increment(id, width, block_idx) };
            }
        }
    }

    checksum
}
