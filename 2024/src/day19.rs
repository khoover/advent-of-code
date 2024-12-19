use super::*;

use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;
use trie_rs::{try_collect::TryFromIterator, Trie, TrieBuilder};

fn make_trie(basis: &str) -> Trie<u8> {
    let mut builder: TrieBuilder<u8> = TrieBuilder::new();

    for towel in basis.split(", ") {
        builder.push(towel);
    }

    builder.build()
}

struct CollectLen(usize);

impl<A> TryFromIterator<A, CollectLen> for CollectLen {
    type Error = ();

    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: IntoIterator<Item = A>,
    {
        Ok(Self(iter.into_iter().count()))
    }
}

#[aoc(day19, part1)]
pub fn part1(s: &str) -> usize {
    let (basis, targets) = s.split_once("\n\n").unwrap();
    let trie = make_trie(basis);

    targets
        .lines()
        .map(|line| line.as_bytes())
        .filter(|&bytes| recursive_check(bytes, &trie))
        .count()
}

fn recursive_check(target: &[u8], basis_trie: &Trie<u8>) -> bool {
    for prefix in basis_trie.common_prefix_search::<CollectLen, _>(target) {
        let new_target = &target[prefix.0..];
        if new_target.is_empty() || recursive_check(new_target, basis_trie) {
            return true;
        }
    }
    false
}

#[aoc(day19, part2)]
pub fn part2(s: &str) -> usize {
    let (basis, targets) = s.split_once("\n\n").unwrap();
    let trie = make_trie(basis);
    let mut count_cache: FxHashMap<ArrayVec<u8, 60>, usize> = FxHashMap::default();

    targets
        .lines()
        .map(|line| line.as_bytes())
        .map(|bytes| recursive_sum(bytes, &trie, &mut count_cache))
        .sum()
}

fn recursive_sum(
    target: &[u8],
    basis_trie: &Trie<u8>,
    cache: &mut FxHashMap<ArrayVec<u8, 60>, usize>,
) -> usize {
    if let Some(cached) = cache.get(target) {
        return *cached;
    }
    let mut sum = 0;
    for prefix in basis_trie.common_prefix_search::<CollectLen, _>(target) {
        let new_target = &target[prefix.0..];
        if new_target.is_empty() {
            sum += 1;
        } else {
            sum += recursive_sum(new_target, basis_trie, cache);
        }
    }
    cache.insert(target.try_into().unwrap(), sum);
    sum
}
