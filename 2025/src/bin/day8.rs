use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use aoc_2025::run_day;
use itertools::Itertools;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn part1(s: &str) -> Result<u64> {
    part1_impl(s, 1000)
}

fn part1_impl(s: &str, pairs: usize) -> Result<u64> {
    let coords: Vec<Coord> = s
        .trim()
        .lines()
        .map(|l| l.parse::<Coord>())
        .collect::<Result<_>>()?;
    let mut disjoint_set = QuickUnionUf::<UnionBySize>::new(coords.len());
    coords
        .iter()
        .copied()
        .enumerate()
        .tuple_combinations::<((usize, Coord), (usize, Coord))>()
        .k_smallest_relaxed_by_key(pairs, |(a, b)| a.1.distance(&b.1))
        .map(|(a, b)| (a.0, b.0))
        .for_each(|(a, b)| {
            disjoint_set.union(a, b);
        });

    Ok((0..disjoint_set.size())
        .map(|idx| {
            let root = disjoint_set.find(idx);
            let size = disjoint_set.get(root).size();
            (root, size)
        })
        .unique_by(|(idx, _)| *idx)
        .map(|(_, x)| x as u64)
        .k_largest_relaxed(3)
        .product())
}

fn part2(s: &str) -> Result<u64> {
    let coords: Vec<Coord> = s
        .trim()
        .lines()
        .map(|l| l.parse::<Coord>())
        .collect::<Result<_>>()?;
    let mut disjoint_set = QuickUnionUf::<UnionBySize>::new(coords.len());
    let mut pair_iter = coords
        .iter()
        .copied()
        .enumerate()
        .tuple_combinations::<((usize, Coord), (usize, Coord))>()
        .sorted_unstable_by_key(|(a, b)| a.1.distance(&b.1));

    let mut connections_remaining = coords.len() - 1;
    loop {
        let pair = pair_iter.next().context("Ran out of pairs")?;
        if disjoint_set.union(pair.0.0, pair.1.0) {
            connections_remaining -= 1;
            if connections_remaining == 0 {
                return Ok(pair.0.1.x as u64 * pair.1.1.x as u64);
            }
        }
    }
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Coord {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Coord {
    pub fn distance(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) as u64).pow(2)
            + (self.y.abs_diff(other.y) as u64).pow(2)
            + (self.z.abs_diff(other.z) as u64).pow(2)
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x_str, yz) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("Malformed input"))?;
        let (y_str, z_str) = yz
            .split_once(',')
            .ok_or_else(|| anyhow!("Malformed input"))?;
        Ok(Self {
            x: x_str.parse()?,
            y: y_str.parse()?,
            z: z_str.parse()?,
        })
    }
}

static INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[test]
fn test_part1() {
    assert_eq!(part1_impl(INPUT, 10).unwrap(), 40);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 25272);
}
