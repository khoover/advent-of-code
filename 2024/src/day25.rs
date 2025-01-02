use std::arch::x86_64::*;

use super::*;

use itertools::{Either, Itertools};

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let (keys, locks) = input
        .split("\n\n")
        .partition_map::<Vec<_>, Vec<_>, _, _, _>(|block| {
            let block = block.as_bytes();
            let actual_range = &block[6..36];
            if block[0] == b'#' {
                let heights: [u8; 5] = std::array::from_fn(|column| {
                    (0..5_u8)
                        .find(|row| actual_range[*row as usize * 6 + column] == b'.')
                        .unwrap_or(5)
                });
                Either::Right(heights)
            } else {
                let heights: [u8; 5] = std::array::from_fn(|column| {
                    5 - (0..5_u8)
                        .find(|row| actual_range[*row as usize * 6 + column] == b'#')
                        .unwrap_or(5)
                });
                Either::Left(heights)
            }
        });
    keys.into_iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| (0..5).all(|idx| key[idx] + lock[idx] <= 5))
                .count()
        })
        .sum()
}

#[aoc(day25, part1, SimdCmp)]
pub fn part1_simd_cmp(input: &str) -> usize {
    let input = input.as_bytes();
    let (keys, locks) = input
        .chunks(43)
        .map(|chunk| (&chunk[0..41]).try_into().unwrap())
        .partition::<Vec<&[u8; 41]>, _>(|chunk: &&[u8; 41]| chunk[0] == b'.');

    unsafe {
        let all_hash = _mm256_broadcastsi128_si256(_mm_set1_epi8(b'#' as i8));
        keys.into_iter()
            .map(|key| {
                let key = _mm256_cmpeq_epi8(
                    all_hash,
                    _mm256_lddqu_si256(&key[4] as *const u8 as *const _),
                );

                locks
                    .iter()
                    .filter(|lock| {
                        let lock = _mm256_cmpeq_epi8(
                            all_hash,
                            _mm256_lddqu_si256(&lock[4] as *const u8 as *const _),
                        );
                        let overlaps = _mm256_movemask_epi8(_mm256_and_si256(key, lock));
                        overlaps == 0
                    })
                    .count()
            })
            .sum()
    }
}

#[aoc(day25, part1, SimdBetterCmp)]
pub fn part1_simd_better_cmp(input: &str) -> usize {
    let input = input.as_bytes();
    let (keys, locks) = input
        .chunks(43)
        .map(|chunk| (&chunk[0..41]).try_into().unwrap())
        .partition::<Vec<&[u8; 41]>, _>(|chunk: &&[u8; 41]| chunk[0] == b'.');

    unsafe {
        let all_hash = _mm256_broadcastsi128_si256(_mm_set1_epi8(b'#' as i8));
        keys.into_iter()
            .map(|key| {
                let key = _mm256_cmpeq_epi8(
                    all_hash,
                    _mm256_lddqu_si256(&key[4] as *const u8 as *const _),
                );

                locks
                    .iter()
                    .filter(|lock| {
                        let lock = _mm256_cmpeq_epi8(
                            all_hash,
                            _mm256_lddqu_si256(&lock[4] as *const u8 as *const _),
                        );
                        _mm256_testz_si256(lock, key) == 1
                    })
                    .count()
            })
            .sum()
    }
}

#[aoc(day25, part1, SimdBuckwild)]
pub fn part1_simd_buckwild(input: &str) -> usize {
    unsafe {
        let all_hash = _mm256_broadcastsi128_si256(_mm_set1_epi8(b'#' as i8));
        let input = input.as_bytes();
        let (keys, locks) = input
            .chunks(43)
            .map(|chunk| (&chunk[0..41]).try_into().unwrap())
            .partition_map::<Vec<_>, Vec<_>, _, _, _>(|chunk: &[u8; 41]| {
                let simd = _mm256_cmpeq_epi8(
                    all_hash,
                    _mm256_lddqu_si256(&chunk[4] as *const u8 as *const _),
                );
                if chunk[0] == b'#' {
                    Either::Left(simd)
                } else {
                    Either::Right(simd)
                }
            });

        keys.into_iter()
            .map(|key| {
                locks
                    .iter()
                    .copied()
                    .map(|lock| _mm256_testz_si256(lock, key) as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}

#[aoc(day25, part1, SimdBuckwildPrealloc)]
pub fn part1_simd_buckwild_prealloc(input: &str) -> usize {
    let input = input.as_bytes();
    let mut keys = Vec::with_capacity(input.len() / 64);
    let mut locks = Vec::with_capacity(input.len() / 64);
    unsafe {
        let all_hash = _mm256_broadcastsi128_si256(_mm_set1_epi8(b'#' as i8));
        input
            .chunks(43)
            .map(|chunk| chunk.get_unchecked(0..41).try_into().unwrap_unchecked())
            .for_each(|chunk: &[u8; 41]| {
                let dest = if chunk[0] == b'#' {
                    &mut keys
                } else {
                    &mut locks
                };
                let simd = _mm256_cmpeq_epi8(
                    all_hash,
                    _mm256_lddqu_si256(&chunk[4] as *const u8 as *const _),
                );
                std::hint::assert_unchecked(dest.capacity() > dest.len());
                dest.insert(dest.len(), simd);
            });

        keys.into_iter()
            .flat_map(|key| {
                locks
                    .iter()
                    .copied()
                    .map(move |lock| _mm256_testz_si256(lock, key) as usize)
            })
            .sum()
    }
}
