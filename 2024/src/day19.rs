use super::*;

use arrayvec::ArrayVec;
use dashmap::DashMap;
use parking_lot::RwLock;
use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap};
use trie_rs::{
    map::{Trie, TrieBuilder},
    try_collect::TryFromIterator,
};

fn make_trie(basis: &str) -> Trie<u8, usize> {
    let mut builder = TrieBuilder::new();

    for towel in basis.split(", ") {
        builder.push(towel, towel.len());
    }

    builder.build()
}

#[aoc_generator(day19)]
fn gen(s: &str) -> (Trie<u8, usize>, String) {
    let (basis, targets) = s.split_once("\n\n").unwrap();
    let trie = make_trie(basis);
    (trie, targets.to_owned())
}

struct DropIter;

impl<A> TryFromIterator<A, DropIter> for DropIter {
    type Error = ();

    fn try_from_iter<T>(_iter: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: IntoIterator<Item = A>,
    {
        Ok(DropIter)
    }
}

#[aoc(day19, part1)]
pub fn part1((trie, targets): &(Trie<u8, usize>, String)) -> usize {
    targets
        .par_lines()
        .map(|line| line.as_bytes())
        .filter(|&bytes| recursive_check(bytes, trie))
        .count()
}

fn recursive_check(target: &[u8], basis_trie: &Trie<u8, usize>) -> bool {
    for (_, prefix_len) in basis_trie.common_prefix_search::<DropIter, _>(target) {
        let new_target = &target[*prefix_len..];
        if new_target.is_empty() || recursive_check(new_target, basis_trie) {
            return true;
        }
    }
    false
}

#[aoc(day19, part2)]
pub fn part2((trie, targets): &(Trie<u8, usize>, String)) -> usize {
    let count_cache: RwLock<FxHashMap<ArrayVec<u8, 60>, usize>> = RwLock::new(FxHashMap::default());

    targets
        .par_lines()
        .map(|line| line.as_bytes())
        .map(|bytes| recursive_sum(bytes, trie, &count_cache))
        .sum()
}

fn recursive_sum(
    target: &[u8],
    basis_trie: &Trie<u8, usize>,
    cache: &RwLock<FxHashMap<ArrayVec<u8, 60>, usize>>,
) -> usize {
    if let Some(cached) = cache.read().get(target) {
        return *cached;
    }
    let mut sum = 0;
    for (_, prefix_len) in basis_trie.common_prefix_search::<DropIter, _>(target) {
        let new_target = &target[*prefix_len..];
        if new_target.is_empty() {
            sum += 1;
        } else {
            sum += recursive_sum(new_target, basis_trie, cache);
        }
    }
    cache.write().insert(target.try_into().unwrap(), sum);
    sum
}

#[aoc(day19, part2, Dash)]
pub fn part2_dash((trie, targets): &(Trie<u8, usize>, String)) -> usize {
    let count_cache: DashMap<ArrayVec<u8, 60>, usize, _> =
        DashMap::with_hasher(FxBuildHasher::default());

    targets
        .par_lines()
        .map(|line| line.as_bytes())
        .map(|bytes| recursive_sum_dash(bytes, trie, &count_cache))
        .sum()
}

fn recursive_sum_dash(
    target: &[u8],
    basis_trie: &Trie<u8, usize>,
    cache: &DashMap<ArrayVec<u8, 60>, usize, FxBuildHasher>,
) -> usize {
    if let Some(cached) = cache.get(target) {
        return *cached;
    }
    let mut sum = 0;
    for (_, prefix_len) in basis_trie.common_prefix_search::<DropIter, _>(target) {
        unsafe {
            std::hint::assert_unchecked(*prefix_len <= target.len());
        }
        let new_target = &target[*prefix_len..];
        if new_target.is_empty() {
            sum += 1;
        } else {
            sum += recursive_sum_dash(new_target, basis_trie, cache);
        }
    }
    cache.insert(unsafe { target.try_into().unwrap_unchecked() }, sum);
    sum
}
