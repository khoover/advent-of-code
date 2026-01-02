use anyhow::{Context, Result};
use aoc_2025::run_day;
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};
use itertools::Itertools;
use regex::Regex;

fn part1(s: &str) -> Result<u64> {
    let lights_re = Regex::new(r"\[(?<inner>[.#]+)\]")?;
    let switches_re = Regex::new(r"\((?<inner>[^)]+)\)")?;
    s.trim()
        .lines()
        .map(|s| {
            let desired_state = lights_re
                .captures(s)
                .context("Missing lights")?
                .name("inner")
                .context("Broken regex")?
                .as_str()
                .as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(|(shift, byte)| if byte == b'#' { 1_u16 << shift } else { 0 })
                .fold(0, |a, b| a | b);
            let switches = switches_re.captures_iter(s).map(|capture| {
                let s = capture.name("inner").context("Broken regex")?.as_str();
                s.split(',')
                    .map(|idx_str| idx_str.parse::<u16>().map(|idx| 1_u16 << idx))
                    .fold_ok(0, |a, b| a | b)
                    .map_err(anyhow::Error::from)
            });

            switches.process_results(|it| {
                it.powerset()
                    .filter_map(|powset| {
                        let flips = powset.len() as u64;
                        let state = powset.into_iter().fold(0, |a, b| a ^ b);
                        (state == desired_state).then_some(flips)
                    })
                    .next()
                    .context("No combo worked")
            })?
        })
        .sum()
}

fn part2(s: &str) -> Result<u64> {
    let switches_re = Regex::new(r"\((?<inner>[^)]+)\)")?;
    let joltage_re = Regex::new(r"\{(?<inner>.+)\}")?;
    let sum: f64 = s
        .trim()
        .lines()
        .map(|s| {
            let switches = switches_re
                .captures_iter(s)
                .map(|capture| capture.name("inner").unwrap().as_str())
                .map(|x| {
                    x.split(',')
                        .map(|idx| idx.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let mut problem = ProblemVariables::new();
            let vars = problem.add_vector(variable().integer().min(0), switches.len());
            let objective: Expression = vars.iter().sum();
            let solution = problem
                .minimise(&objective)
                .using(default_solver)
                .with_all(
                    joltage_re
                        .captures(s)
                        .unwrap()
                        .name("inner")
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|x| x.parse::<u16>().unwrap())
                        .enumerate()
                        .map(|(idx, x)| {
                            switches
                                .iter()
                                .enumerate()
                                .filter_map(|(switch_idx, v)| {
                                    v.contains(&idx).then_some(switch_idx)
                                })
                                .map(|var_idx| &vars[var_idx])
                                .sum::<Expression>()
                                .eq(x)
                        }),
                )
                .solve()?;
            Ok(solution.eval(objective))
        })
        .sum::<Result<_>>()?;
    Ok(sum.trunc() as u64)
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

static INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT).unwrap(), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT).unwrap(), 33);
}
