use std::{fmt::Display, fs::read_to_string, path::PathBuf, time::Instant};

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command, value_parser};

pub mod byte_grid;
pub mod byte_search;

pub fn run_day<F1, R1, F2, R2>(part1: F1, part2: F2) -> Result<()>
where
    R1: Display,
    F1: for<'a> FnOnce(&'a str) -> Result<R1>,
    R2: Display,
    F2: for<'a> FnOnce(&'a str) -> Result<R2>,
{
    let matches = Command::new("AOC Runner")
        .arg(
            Arg::new("input_path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .arg(
            Arg::new("part_two")
                .long("part_two")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let path = matches
        .get_one::<PathBuf>("input_path")
        .context("Failed to parse path from command invocation")?;
    let s =
        read_to_string(path).with_context(|| format!("Failed to read from {}", path.display()))?;
    if matches.get_flag("part_two") {
        let start = Instant::now();
        let res = part2(&s);
        let duration = (Instant::now() - start).as_secs_f64();
        render_result_and_duration(duration, res)
    } else {
        let start = Instant::now();
        let res = part1(&s);
        let duration = (Instant::now() - start).as_secs_f64();
        render_result_and_duration(duration, res)
    }
}

fn render_result_and_duration<T: Display>(duration: f64, res: Result<T>) -> Result<()> {
    if duration > 5.0 {
        println!("Duration: {:.3}s", duration);
    } else {
        println!("Duration: {:.3}ms", duration * 1000.0);
    }
    res.map(|r| {
        println!("{}", r);
    })
}
