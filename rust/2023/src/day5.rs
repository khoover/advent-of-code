use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{i64, newline, space1},
    combinator::{all_consuming, verify},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    Finish, IResult, Parser,
};
use std::cmp::Reverse;
use std::ops::Range;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Debug, PartialEq, Eq)]
struct RangeOffset {
    bounds: Range<i64>,
    offset: i64,
}

impl PartialOrd for RangeOffset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeOffset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bounds.start.cmp(&other.bounds.start).then(
            self.bounds
                .end
                .cmp(&other.bounds.end)
                .then(self.offset.cmp(&other.offset)),
        )
    }
}

impl RangeOffset {
    fn nom(input: &str) -> IResult<&str, Self> {
        (i64, space1, i64, space1, i64)
            .map(|(dest_start, _, source_start, _, len)| Self {
                offset: dest_start - source_start,
                bounds: source_start..(source_start + len),
            })
            .parse(input)
    }

    fn map(&self, source: i64) -> Option<i64> {
        self.bounds
            .contains(&source)
            .then_some(source + self.offset)
    }
}

#[derive(Clone, Debug)]
struct RangeMap {
    ranges: Vec<RangeOffset>,
}

impl RangeMap {
    fn new(ranges: Vec<RangeOffset>) -> Self {
        let mut heap: BinaryHeap<Reverse<RangeOffset>> = ranges.into_iter().map(Reverse).collect();
        let mut ranges = Vec::with_capacity(heap.len() * 2);
        let Some(Reverse(mut prev)) = heap.pop() else {
            return Self { ranges };
        };
        if prev.bounds.start != 0 {
            ranges.push(RangeOffset {
                offset: 0,
                bounds: 0..prev.bounds.start,
            });
        }
        ranges.push(prev.clone());

        while let Some(Reverse(next)) = heap.pop() {
            if prev.bounds.end < next.bounds.start {
                ranges.push(RangeOffset {
                    offset: 0,
                    bounds: prev.bounds.end..next.bounds.start,
                });
            }
            ranges.push(next.clone());
            prev = next;
        }

        Self { ranges }
    }

    fn get_containing_range(&self, source: i64) -> Option<&RangeOffset> {
        let idx = self
            .ranges
            .partition_point(|range| range.bounds.start <= source);
        let range = &self.ranges[idx - 1];
        range.bounds.contains(&source).then_some(range)
    }

    fn map(&self, source: i64) -> i64 {
        self.get_containing_range(source)
            .map(|range| range.map(source).expect("Bug in get_containing_range"))
            .unwrap_or(source)
    }

    fn nom(input: &str) -> IResult<&str, Self> {
        separated_list0(newline, RangeOffset::nom)
            .map(Self::new)
            .parse(input)
    }
}

fn range_map_with_names(input: &str) -> IResult<&str, (&str, &str, RangeMap)> {
    separated_pair(
        separated_pair(take_until("-"), tag("-to-"), take_until(" map:")),
        (tag(" map:"), newline),
        RangeMap::nom,
    )
    .map(|((a, b), c)| (a, b, c))
    .parse(input)
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: ArrayVec<RangeMap, 7>,
}

impl Almanac {
    fn nom(input: &str) -> IResult<&str, Self> {
        separated_pair(
            preceded(tag("seeds: "), separated_list1(space1, i64)),
            (newline, newline),
            verify(
                separated_list1((newline, newline), range_map_with_names),
                |v: &[_]| {
                    v.len() == 7
                        && v[0].0 == "seed"
                        && v[6].1 == "location"
                        && v.windows(2).all(|x| x[0].1 == x[1].0)
                },
            ),
        )
        .map(|(seeds, maps_with_names)| Self {
            seeds,
            maps: maps_with_names.into_iter().map(|(_, _, x)| x).collect(),
        })
        .parse(input)
    }
}

#[aoc_generator(day5)]
fn day5_gen(input: &str) -> Result<Almanac> {
    all_consuming(Almanac::nom)
        .parse_complete(input)
        .finish()
        .map(|(_, x)| x)
        .map_err(|e| Error::msg(format!("Failed to parse input: {e}")))
}

#[aoc(day5, part1)]
fn day5_part1(input: &Almanac) -> Result<i64> {
    input
        .seeds
        .iter()
        .copied()
        .map(|seed| input.maps.iter().fold(seed, |source, map| map.map(source)))
        .min()
        .context("input was empty")
}

#[aoc(day5, part2)]
fn day5_part2(input: &Almanac) -> Result<i64> {
    let initial_ranges: Vec<_> = input
        .seeds
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[1]))
        .collect();
    let output_location_ranges: Vec<_> =
        input
            .maps
            .iter()
            .fold(initial_ranges, |source_ranges, map| {
                source_ranges
                    .into_iter()
                    .flat_map(|source_range| {
                        ((map
                            .ranges
                            .partition_point(|range| range.bounds.start <= source_range.0)
                            - 1)..)
                            .scan(Some(source_range), |maybe_source, idx| {
                                if idx >= map.ranges.len() {
                                    return maybe_source.take();
                                }
                                let Some((start, len)) = maybe_source else {
                                    return None;
                                };
                                if *len == 0 {
                                    return None;
                                }

                                let range = &map.ranges[idx];
                                let dest_start = *start + range.offset;
                                let dest_len = (range.bounds.end - *start).min(*len);
                                *start += dest_len;
                                *len -= dest_len;
                                Some((dest_start, dest_len))
                            })
                    })
                    .collect()
            });
    output_location_ranges
        .into_iter()
        .map(|(start, _)| start)
        .min()
        .context("input was empty")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_map_new() {
        let ranges = vec![
            RangeOffset {
                offset: 49 - 53,
                bounds: 53..(53 + 8),
            },
            RangeOffset {
                offset: 0 - 12,
                bounds: 12..(12 + 41),
            },
            RangeOffset {
                offset: 42 - 0,
                bounds: 0..(0 + 7),
            },
            RangeOffset {
                offset: 57 - 7,
                bounds: 7..(7 + 4),
            },
        ];
        let map = RangeMap::new(ranges);
        assert_eq!(
            map.ranges,
            &[
                RangeOffset {
                    offset: 42 - 0,
                    bounds: 0..(0 + 7)
                },
                RangeOffset {
                    offset: 57 - 7,
                    bounds: 7..(7 + 4)
                },
                RangeOffset {
                    offset: 0,
                    bounds: 11..12
                },
                RangeOffset {
                    offset: 0 - 12,
                    bounds: 12..(12 + 41)
                },
                RangeOffset {
                    offset: 49 - 53,
                    bounds: 53..(53 + 8)
                }
            ]
        )
    }
}
