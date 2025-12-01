use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Arg, Command, value_parser};

pub fn run_day<F, R>(f: F) -> Result<()>
where
    R: Display,
    F: for<'a> FnOnce(&'a str) -> Result<R>,
{
    let matches = Command::new("AOC Runner")
        .arg(Arg::new("input_path").value_parser(value_parser!(PathBuf)))
        .get_matches();
    let path = matches
        .get_one::<PathBuf>("input_path")
        .context("Failed to parse path from command invocation")?;
    let s =
        read_to_string(path).with_context(|| format!("Failed to read from {}", path.display()))?;
    f(&s).map(|r| {
        println!("{}", r);
    })
}
