use std::collections::hash_map::Entry;

use super::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[aoc(day22, part1)]
pub fn part1(s: &str) -> u64 {
    s.par_lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|initial_secret| (0..2000).fold(initial_secret, |secret, _| evolve(secret)))
        .sum()
}

#[inline(always)]
fn evolve(mut secret: u64) -> u64 {
    secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

#[aoc(day22, part2)]
pub fn part2(s: &str) -> u64 {
    let total_profits = Mutex::new(FxHashMap::<[i8; 4], u64>::default());
    s.par_lines()
        .map(|line| line.parse::<u64>().unwrap())
        .for_each(|mut secret| {
            let mut local_profits = FxHashMap::<[i8; 4], i8>::default();

            let mut previous_price = (secret % 10) as i8;
            secret = evolve(secret);
            let mut price = (secret % 10) as i8;
            let mut diffs = [0_i8; 4];
            diffs[0] = price - previous_price;
            previous_price = price;

            secret = evolve(secret);
            price = (secret % 10) as i8;
            diffs[1] = price - previous_price;
            previous_price = price;

            secret = evolve(secret);
            price = (secret % 10) as i8;
            diffs[2] = price - previous_price;
            previous_price = price;

            secret = evolve(secret);
            price = (secret % 10) as i8;
            diffs[3] = price - previous_price;
            local_profits.insert(diffs, price);
            previous_price = price;

            for _ in 4..2000 {
                secret = evolve(secret);
                price = (secret % 10) as i8;
                diffs = [diffs[1], diffs[2], diffs[3], price - previous_price];
                if let Entry::Vacant(entry) = local_profits.entry(diffs) {
                    entry.insert(price);
                }
                previous_price = price;
            }

            let mut total_profits = total_profits.lock();
            for (diffs, price) in local_profits.into_iter() {
                total_profits
                    .entry(diffs)
                    .and_modify(|profit| *profit += price as u64)
                    .or_insert(price as u64);
            }
        });
    total_profits
        .into_inner()
        .into_iter()
        .map(|(_, v)| v)
        .max()
        .unwrap()
}

#[aoc(day22, part2, Scope)]
pub fn part2_scope(input: &str) -> u64 {
    let mut total_profits = FxHashMap::<[i8; 4], u64>::default();

    rayon::scope(|s| {
        let (sender, receiver) = std::sync::mpsc::channel::<FxHashMap<[i8; 4], i8>>();
        let total_profits = &mut total_profits;
        s.spawn(move |_| {
            receiver.iter().for_each(|local_profits| {
                for (diffs, price) in local_profits.into_iter() {
                    total_profits
                        .entry(diffs)
                        .and_modify(|profit| *profit += price as u64)
                        .or_insert(price as u64);
                }
            })
        });
        input
            .par_lines()
            .map(|line| line.parse::<u64>().unwrap())
            .for_each(|mut secret| {
                let mut local_profits = FxHashMap::<[i8; 4], i8>::default();

                let mut previous_price = (secret % 10) as i8;
                secret = evolve(secret);
                let mut price = (secret % 10) as i8;
                let mut diffs = [0_i8; 4];
                diffs[0] = price - previous_price;
                previous_price = price;

                secret = evolve(secret);
                price = (secret % 10) as i8;
                diffs[1] = price - previous_price;
                previous_price = price;

                secret = evolve(secret);
                price = (secret % 10) as i8;
                diffs[2] = price - previous_price;
                previous_price = price;

                secret = evolve(secret);
                price = (secret % 10) as i8;
                diffs[3] = price - previous_price;
                local_profits.insert(diffs, price);
                previous_price = price;

                for _ in 4..2000 {
                    secret = evolve(secret);
                    price = (secret % 10) as i8;
                    diffs = [diffs[1], diffs[2], diffs[3], price - previous_price];
                    if let Entry::Vacant(entry) = local_profits.entry(diffs) {
                        entry.insert(price);
                    }
                    previous_price = price;
                }

                let _ = sender.send(local_profits);
            });
    });

    total_profits.into_iter().map(|(_, v)| v).max().unwrap()
}
