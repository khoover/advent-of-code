use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};
use rustc_hash::FxHashSet as HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymbolOrNum {
    Symbol(char),
    Num(u32),
}

#[aoc_generator(day3)]
fn day3_gen(input: &str) -> Result<(Quadtree<u16, SymbolOrNum>, Vec<u64>)> {
    let mut qt = Quadtree::new(8);
    let mut points = Vec::new();

    input.lines().enumerate().try_for_each(|(row_idx, line)| {
        let end = line
            .chars()
            .enumerate()
            .try_fold(None, |maybe_partial, (col_idx, next)| {
                Ok::<std::option::Option<(usize, u32)>, Error>(
                    match (maybe_partial, next.to_digit(10)) {
                        (None, Some(x)) => Some((col_idx, x)),
                        (Some((idx, prev)), Some(x)) => Some((idx, prev * 10 + x)),
                        (Some((idx, prev)), None) => {
                            if next != '.' {
                                let handle = qt
                                    .insert_pt(
                                        Point {
                                            x: row_idx as u16 + 1,
                                            y: col_idx as u16 + 1,
                                        },
                                        SymbolOrNum::Symbol(next),
                                    )
                                    .context("Failed to insert point")?;
                                points.push(handle);
                            }
                            let number_region = AreaBuilder::default()
                                .anchor(Point {
                                    x: row_idx as u16 + 1,
                                    y: idx as u16 + 1,
                                })
                                .dimensions((1, (col_idx - idx) as u16))
                                .build()
                                .map_err(Error::msg)?;
                            qt.insert(number_region, SymbolOrNum::Num(prev))
                                .context("Failed to insert region")?;
                            None
                        }
                        (None, None) => {
                            if next != '.' {
                                let handle = qt
                                    .insert_pt(
                                        Point {
                                            x: row_idx as u16 + 1,
                                            y: col_idx as u16 + 1,
                                        },
                                        SymbolOrNum::Symbol(next),
                                    )
                                    .context("Failed to insert point")?;
                                points.push(handle);
                            }
                            None
                        }
                    },
                )
            })?;

        if let Some((start, num)) = end {
            let region = AreaBuilder::default()
                .anchor(Point {
                    x: row_idx as u16 + 1,
                    y: start as u16 + 1,
                })
                .dimensions((1, line.len() as u16 - start as u16))
                .build()
                .map_err(Error::msg)?;
            qt.insert(region, SymbolOrNum::Num(num))
                .context("Failed to insert region")?;
        }

        Ok::<(), Error>(())
    })?;

    Ok((qt, points))
}

#[aoc(day3, part1)]
fn day3_part1(input: &(Quadtree<u16, SymbolOrNum>, Vec<u64>)) -> Result<u32> {
    let (qt, symbols) = input;
    let mut seen = HashSet::default();
    let mut sum = 0;
    for handle in symbols {
        let location = qt
            .get(*handle)
            .context("Missing previously inserted handle")?
            .anchor();
        let query = AreaBuilder::default()
            .anchor(Point {
                x: location.x - 1,
                y: location.y - 1,
            })
            .dimensions((3, 3))
            .build()
            .map_err(Error::msg)?;
        for entry in qt.query(query) {
            if let SymbolOrNum::Num(x) = entry.value_ref() {
                if seen.insert(entry.handle()) {
                    sum += x;
                }
            }
        }
    }

    Ok(sum)
}

#[aoc(day3, part2)]
fn day3_part2(input: &(Quadtree<u16, SymbolOrNum>, Vec<u64>)) -> Result<u64> {
    let (qt, symbols) = input;
    symbols
        .iter()
        .copied()
        .filter_map(|handle| match qt.get(handle) {
            None => Some(Err(Error::msg("Missing previously inserted handle"))),
            Some(entry) if *entry.value_ref() == SymbolOrNum::Symbol('*') => Some(Ok(entry)),
            _ => None,
        })
        .map(|entry_res| {
            let location = entry_res?.anchor();
            AreaBuilder::default()
                .anchor(Point {
                    x: location.x - 1,
                    y: location.y - 1,
                })
                .dimensions((3, 3))
                .build()
                .map_err(Error::msg)
        })
        .filter_map(|query_res| {
            let query = match query_res {
                Ok(q) => q,
                Err(e) => return Some(Err(e)),
            };
            let mut iter = qt.query(query).filter_map(|entry| match entry.value_ref() {
                SymbolOrNum::Num(x) => Some(*x),
                _ => None,
            });
            let first = iter.next()?;
            let second = iter.next()?;
            if iter.next().is_some() {
                None
            } else {
                Some(Ok(first as u64 * second as u64))
            }
        })
        .sum()
}
