use std::collections::{HashSet, VecDeque};

use anyhow::{Context, Result};
use aoc_2025::run_day;
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};
use regex::Regex;

fn part1(s: &str) -> Result<u64> {
    let lights_re = Regex::new(r"\[(?<inner>[.#]+)\]")?;
    let switches_re = Regex::new(r"\((?<inner>[^)]+)\)")?;
    s.trim()
        .lines()
        .map(|s| {
            let lights = lights_re
                .captures(s)
                .context("Missing lights")?
                .name("inner")
                .unwrap()
                .as_str();
            let switches = switches_re
                .captures_iter(s)
                .map(|capture| capture.name("inner").unwrap().as_str());
            Machine::from_regex_captures(lights, switches)
        })
        .map(|r| r.map(|m| m.solve_min_flips()))
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    pub light_state: u16,
    pub desired_light_state: u16,
    pub switches: Vec<u16>,
}

impl Machine {
    fn from_regex_captures<'a>(
        desired_light_state: &str,
        switches: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self> {
        let desired_light_state: u16 = desired_light_state
            .as_bytes()
            .iter()
            .copied()
            .enumerate()
            .map(|(shift, byte)| if byte == b'#' { 1_u16 << shift } else { 0 })
            .reduce(|a, b| a | b)
            .context("Expected non-empty string")?;
        let switches: Vec<u16> = switches
            .into_iter()
            .map(|s| {
                s.split(',')
                    .map(|idx_str| Ok(1_u16 << idx_str.parse::<u16>()?))
                    .sum()
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            light_state: 0,
            desired_light_state,
            switches,
        })
    }

    fn solve_min_flips(&self) -> u64 {
        let mut search_spaces: VecDeque<(u16, u64)> = VecDeque::new();
        search_spaces.push_back((0, 0));
        let mut seen = HashSet::new();
        loop {
            let (state, flips) = search_spaces.pop_front().unwrap();
            if !seen.insert(state) {
                continue;
            }
            for &switch in self.switches.iter() {
                let new_state = switch ^ state;
                if new_state == self.desired_light_state {
                    return flips + 1;
                } else {
                    search_spaces.push_back((new_state, flips + 1));
                }
            }
        }
    }
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
