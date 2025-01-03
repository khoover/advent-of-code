use super::*;
use itertools::{Either, Itertools};
use std::arch::x86_64::*;
use std::sync::Mutex;

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
                if chunk[0] == b'.' {
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
                let dest = if chunk[0] == b'.' {
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

#[aoc(day25, part1, SimdBuckwildPreallocFilter)]
pub fn part1_simd_buckwild_prealloc_filter(input: &str) -> usize {
    let input = input.as_bytes();
    let mut keys = Vec::with_capacity(input.len() / 64);
    let mut locks = Vec::with_capacity(input.len() / 64);
    unsafe {
        let all_hash = _mm256_broadcastsi128_si256(_mm_set1_epi8(b'#' as i8));
        input
            .chunks(43)
            .map(|chunk| chunk.get_unchecked(0..41).try_into().unwrap_unchecked())
            .for_each(|chunk: &[u8; 41]| {
                let dest = if chunk[0] == b'.' {
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
            .map(|key| {
                locks
                    .iter()
                    .filter(move |lock| _mm256_testz_si256(**lock, key) != 0)
                    .count()
            })
            .sum()
    }
}

#[repr(align(32))]
struct DataArray([u32; 256]);

struct Data {
    keys: DataArray,
    locks: DataArray,
}

static DATA: Mutex<Data> = Mutex::new(Data {
    keys: DataArray([u32::MAX; 256]),
    locks: DataArray([u32::MAX; 256]),
});

#[aoc(day25, part1, SimdBigAssumptions)]
pub fn part1_simd_big_assumptions(input: &str) -> usize {
    let input = input.as_bytes();
    let mut lock = DATA.lock().unwrap();
    let data: &mut Data = &mut *lock;
    let keys = &mut data.keys.0;
    let locks = &mut data.locks.0;

    let mut key_len = 0;
    let mut lock_len = 0;

    unsafe {
        let all_hash = _mm256_broadcastsi128_si256(_mm_set1_epi8(b'#' as i8));
        for chunk in input.chunks(43) {
            let dest = if *chunk.get_unchecked(0) == b'.' {
                let res = keys.get_unchecked_mut(key_len);
                key_len += 1;
                res
            } else {
                let res = locks.get_unchecked_mut(lock_len);
                lock_len += 1;
                res
            };
            let simd = _mm256_cmpeq_epi8(
                all_hash,
                _mm256_lddqu_si256(chunk.get_unchecked(4) as *const u8 as *const _),
            );
            *dest = _mm256_movemask_epi8(simd) as u32;
        }

        let zeros = _mm256_setzero_si256();
        let mut counts = _mm256_setzero_si256();
        let mut lock_chunks = locks.chunks_exact(8 * 12);
        for lock_chunk in lock_chunks.by_ref() {
            for i in 0..250 {
                let key = _mm256_set1_epi32(*keys.get_unchecked(i) as i32);
                for chunk in lock_chunk.chunks_exact(8) {
                    let lock_chunk = _mm256_load_si256(chunk.as_ptr() as *const _);
                    let valid = _mm256_cmpeq_epi32(zeros, _mm256_and_si256(key, lock_chunk));
                    counts = _mm256_sub_epi32(counts, valid);
                }
            }
        }

        for i in 0..250 {
            let key = _mm256_set1_epi32(*keys.get_unchecked(i) as i32);
            for chunk in lock_chunks.remainder().chunks_exact(8) {
                let lock_chunk = _mm256_load_si256(chunk.as_ptr() as *const _);
                let valid = _mm256_cmpeq_epi32(zeros, _mm256_and_si256(key, lock_chunk));
                counts = _mm256_sub_epi32(counts, valid);
            }
        }

        let counts_low = _mm256_extracti128_si256::<0>(counts);
        let counts_high = _mm256_extracti128_si256::<1>(counts);
        let counts_quad = _mm_hadd_epi32(counts_low, counts_high);
        let counts_duo = _mm_hadd_epi32(counts_quad, counts_quad);
        let count = _mm_hadd_epi32(counts_duo, counts_duo);
        _mm_cvtsi128_si32(count) as u32 as usize
    }
}
