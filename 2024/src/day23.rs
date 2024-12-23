use super::*;
use rustc_hash::FxHashSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const fn byte_pair_to_idx(first: u8, second: u8) -> usize {
    unsafe {
        std::hint::assert_unchecked(b'a' <= first && first <= b'z');
        std::hint::assert_unchecked(b'a' <= second && second <= b'z');
    }
    (first - b'a') as usize * 32 + (second - b'a') as usize
}

const fn idx_to_byte_pair(idx: usize) -> [u8; 2] {
    let second = (idx % 32) as u8 + b'a';
    let first = (idx / 32) as u8 + b'a';
    [first, second]
}

const TA_INDEX: usize = byte_pair_to_idx(b't', b'a');
const TZ_INDEX: usize = byte_pair_to_idx(b't', b'z');
const UPPER: usize = 25 * 32 + 25 + 1;

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let adjacency_matrix: [[AtomicBool; UPPER]; UPPER] =
        unsafe { std::mem::transmute([[false; UPPER]; UPPER]) };
    input.par_lines().for_each(|line| {
        let line = line.as_bytes();
        unsafe {
            std::hint::assert_unchecked(line.len() == 5);
        }
        let idx_a = byte_pair_to_idx(line[0], line[1]);
        let idx_b = byte_pair_to_idx(line[3], line[4]);
        adjacency_matrix[idx_a][idx_b].store(true, Ordering::Relaxed);
        adjacency_matrix[idx_b][idx_a].store(true, Ordering::Relaxed);
    });

    (b'a'..=b'z')
        .into_par_iter()
        .flat_map(|first| {
            (b'a'..=b'z')
                .into_par_iter()
                .map(move |second| byte_pair_to_idx(first, second))
        })
        .flat_map(|idx_a| {
            let adjacency_matrix = &adjacency_matrix;
            (idx_a + 1..UPPER).into_par_iter().map(move |idx_b| {
                (idx_b + 1..UPPER)
                    .filter(move |&idx_c| {
                        let range = TA_INDEX..=TZ_INDEX;
                        adjacency_matrix[idx_a][idx_b].load(Ordering::Relaxed)
                            && adjacency_matrix[idx_a][idx_c].load(Ordering::Relaxed)
                            && adjacency_matrix[idx_b][idx_c].load(Ordering::Relaxed)
                            && (range.contains(&idx_a)
                                || range.contains(&idx_b)
                                || range.contains(&idx_c))
                    })
                    .count()
            })
        })
        .sum()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let adjacency_matrix: [[AtomicBool; UPPER]; UPPER] =
        unsafe { std::mem::transmute([[false; UPPER]; UPPER]) };
    input.par_lines().for_each(|line| {
        let line = line.as_bytes();
        unsafe {
            std::hint::assert_unchecked(line.len() == 5);
        }
        let idx_a = byte_pair_to_idx(line[0], line[1]);
        let idx_b = byte_pair_to_idx(line[3], line[4]);
        adjacency_matrix[idx_a][idx_b].store(true, Ordering::Relaxed);
        adjacency_matrix[idx_b][idx_a].store(true, Ordering::Relaxed);
    });
    let adjacency_lists: Vec<FxHashSet<usize>> = adjacency_matrix
        .into_par_iter()
        .map(|matrix_row| {
            matrix_row
                .into_iter()
                .enumerate()
                .filter_map(|(idx, edge)| edge.load(Ordering::Relaxed).then_some(idx))
                .collect()
        })
        .collect();

    let max_clique = AtomicUsize::new(0);
    rayon::scope(|spawner| {
        let max_clique = &max_clique;
        let adjacency_lists = adjacency_lists.as_slice();
        let vertexes: Vec<_> = (b'a'..=b'z')
            .into_par_iter()
            .flat_map(|first| {
                (b'a'..=b'z')
                    .into_par_iter()
                    .map(move |second| byte_pair_to_idx(first, second))
            })
            .collect();
        let mut p: FxHashSet<_> = vertexes.iter().copied().collect();
        let mut x = FxHashSet::default();
        for vertex in vertexes {
            let r = FxHashSet::from_iter([vertex]);
            let new_p = p.intersection(&adjacency_lists[vertex]).copied().collect();
            let new_x = x.intersection(&adjacency_lists[vertex]).copied().collect();
            spawner.spawn(move |_| bron_kerbosch(max_clique, adjacency_lists, r, new_p, new_x));
            p.remove(&vertex);
            x.insert(vertex);
        }
    });
    max_clique.load(Ordering::Relaxed)
}

fn bron_kerbosch(
    max_clique: &AtomicUsize,
    adjacency_lists: &[FxHashSet<usize>],
    r: FxHashSet<usize>,
    mut p: FxHashSet<usize>,
    mut x: FxHashSet<usize>,
) {
    if p.is_empty() {
        if x.is_empty() {
            if max_clique.fetch_max(r.len(), Ordering::Relaxed) < r.len() {
                let mut indexes = r.into_iter().collect::<Vec<_>>();
                indexes.sort_unstable();
                indexes.into_iter().map(idx_to_byte_pair).for_each(|pair| {
                    print!("{},", unsafe { std::str::from_utf8_unchecked(&pair) });
                });
                println!("");
            }
        }
        return;
    }
    let vertexes: Vec<_> = p.iter().copied().collect();
    for vertex in vertexes {
        let mut new_r = r.clone();
        new_r.insert(vertex);
        let new_p = p.intersection(&adjacency_lists[vertex]).copied().collect();
        let new_x = x.intersection(&adjacency_lists[vertex]).copied().collect();
        bron_kerbosch(max_clique, adjacency_lists, new_r, new_p, new_x);
        p.remove(&vertex);
        x.insert(vertex);
    }
}
