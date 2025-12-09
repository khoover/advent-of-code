use anyhow::{Context, Error as AnyhowError, Result, anyhow};
use aoc_2025::run_day;
use itertools::Itertools;
use quadtree_rs::area::{Area, AreaBuilder};

fn part1(s: &str) -> Result<u64> {
    let points = s
        .trim()
        .lines()
        .map(|l| {
            let (a, b) = l
                .split_once(',')
                .with_context(|| format!("Malformed line: {}", l))?;
            Ok::<_, AnyhowError>((a.parse()?, b.parse()?))
        })
        .process_results(|it| get_column_minmax(it))?;

    let bot_left_top_right_best: u64 = {
        let mut max_area = 0;
        let mut max_yl = 0;
        for (idx, (xl, (_, yl))) in points.iter().copied().enumerate() {
            if yl <= max_yl {
                continue;
            }
            max_yl = yl;
            let mut min_yr = u64::MAX;
            for (xr, (yr, _)) in points[idx + 1..].iter().rev().copied() {
                if yr < min_yr {
                    min_yr = yr;
                    max_area = max_area.max(get_area((xl, yl), (xr, yr)));
                }
            }
        }
        max_area
    };
    let top_left_bot_right_best: u64 = {
        let mut max_area = 0;
        let mut min_yl = u64::MAX;
        for (idx, (xl, (yl, _))) in points.iter().copied().enumerate() {
            if yl >= min_yl {
                continue;
            }
            min_yl = yl;
            let mut max_yr = 0;
            for (xr, (_, yr)) in points[idx + 1..].iter().rev().copied() {
                if yr > max_yr {
                    max_yr = yr;
                    max_area = max_area.max(get_area((xl, yl), (xr, yr)));
                }
            }
        }
        max_area
    };
    Ok(bot_left_top_right_best.max(top_left_bot_right_best))
}

fn get_column_minmax(it: impl IntoIterator<Item = (u64, u64)>) -> Vec<(u64, (u64, u64))> {
    it.into_iter()
        .into_grouping_map_by(|(x, _)| *x)
        .minmax_by_key(|_, (_, y)| *y)
        .into_iter()
        .filter_map(|(x, res)| Some((x, res.into_option().map(|(a, b)| (a.1, b.1))?)))
        .sorted_unstable_by_key(|(x, _)| *x)
        .collect()
}

fn get_area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn part2(s: &str) -> Result<u64> {
    let points: Vec<(u32, u32)> = s
        .trim()
        .lines()
        .map(|l| {
            let (a, b) = l
                .split_once(',')
                .with_context(|| format!("Malformed line: {}", l))?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<_>>()?;

    let edges: Vec<Area<u32>> = points
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(a, b)| {
            let anchor = (a.0.min(b.0), a.1.min(b.1));
            let dimensions = (a.0.abs_diff(b.0) + 1, a.1.abs_diff(b.1) + 1);
            AreaBuilder::default()
                .anchor(anchor.into())
                .dimensions(dimensions)
                .build()
                .map_err(|s| anyhow!(s))
        })
        .collect::<Result<_>>()?;

    points
        .iter()
        .copied()
        .tuple_combinations::<(_, _)>()
        .filter(|&(a, b)| {
            let Some(interior) = get_interior(a, b) else {
                return true;
            };
            !edges.iter().any(|area| area.intersects(interior))
        })
        .map(|(a, b)| get_area((a.0 as u64, a.1 as u64), (b.0 as u64, b.1 as u64)))
        .max()
        .context("Should have at least one rectange")
}

fn get_interior(a: (u32, u32), b: (u32, u32)) -> Option<Area<u32>> {
    let anchor = (a.0.min(b.0) + 1, a.1.min(b.1) + 1);
    let dimensions = (
        a.0.abs_diff(b.0).checked_sub(1)?,
        a.1.abs_diff(b.1).checked_sub(1)?,
    );
    AreaBuilder::default()
        .anchor(anchor.into())
        .dimensions(dimensions)
        .build()
        .ok()
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 50);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 24);
}
