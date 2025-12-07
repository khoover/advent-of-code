use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command, value_parser};

pub mod byte_grid;

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
        part2(&s).map(|r| {
            println!("{}", r);
        })
    } else {
        part1(&s).map(|r| {
            println!("{}", r);
        })
    }
}
