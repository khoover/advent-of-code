use super::*;
use rustc_hash::FxHashSet;

fn score_dfs2(pos: usize, stride: usize, input: &[u8]) -> usize {
    fn dfs_recursion2(pos: usize, stride: usize, input: &[u8], current: u8) -> usize {
        if current == b'9' {
            return 1;
        }

        let next: u8 = current + 1;
        let mut found = 0;
        for offset in [-(stride as isize), 1, -1, stride as isize] {
            if let Some(next_pos) = pos
                .checked_add_signed(offset)
                .filter(|&idx| idx < input.len())
            {
                if unsafe { *input.get_unchecked(next_pos) } == next {
                    found += dfs_recursion2(next_pos, stride, input, next);
                }
            }
        }

        found
    }

    dfs_recursion2(pos, stride, input, b'0')
}

#[aoc(day10, part2)]
pub fn part2(s: &str) -> usize {
    let stride = s.find("\n").unwrap() + 1;
    let input = s.as_bytes();
    input
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, b)| *b == b'0')
        .map(|(idx, _)| score_dfs2(idx, stride, input))
        .sum()
}

fn score_dfs(pos: usize, stride: usize, input: &[u8], reachable: &mut FxHashSet<usize>) {
    fn dfs_recursion(
        pos: usize,
        stride: usize,
        input: &[u8],
        current: u8,
        reachable: &mut FxHashSet<usize>,
    ) {
        if current == b'9' {
            reachable.insert(pos);
            return;
        }

        let next: u8 = current + 1;
        for offset in [-(stride as isize), 1, -1, stride as isize] {
            if let Some(next_pos) = pos
                .checked_add_signed(offset)
                .filter(|&idx| idx < input.len())
            {
                if unsafe { *input.get_unchecked(next_pos) } == next {
                    dfs_recursion(next_pos, stride, input, next, reachable);
                }
            }
        }
    }

    dfs_recursion(pos, stride, input, b'0', reachable);
}

#[aoc(day10, part1)]
pub fn part1(s: &str) -> usize {
    let stride = s.find("\n").unwrap() + 1;
    let input = s.as_bytes();
    let mut reachable = FxHashSet::default();
    input
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, b)| *b == b'0')
        .map(|(idx, _)| {
            reachable.clear();
            score_dfs(idx, stride, input, &mut reachable);
            reachable.len()
        })
        .sum()
}
