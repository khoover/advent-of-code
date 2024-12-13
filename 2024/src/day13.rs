use super::*;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::{iterator, opt},
    sequence::{preceded, separated_pair, terminated},
    Parser,
};

#[derive(Clone, Copy, Debug)]
struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn xy_pair_parser(s: &str) -> StrIResult<'_, (u64, u64)> {
    separated_pair(
        preceded(tag("X+"), u64),
        tag(", "),
        preceded(tag("Y+"), u64),
    )
    .parse(s)
}

impl Machine {
    fn nom(s: &str) -> StrIResult<'_, Self> {
        let button_a_parser = preceded(tag("Button A: "), xy_pair_parser);
        let button_b_parser = preceded(tag("Button B: "), xy_pair_parser);
        let prize_parser = preceded(
            tag("Prize: "),
            separated_pair(
                preceded(tag("X="), u64),
                tag(", "),
                preceded(tag("Y="), u64),
            ),
        );
        (
            button_a_parser,
            newline,
            button_b_parser,
            newline,
            prize_parser,
        )
            .map(|(button_a, _, button_b, _, prize)| Self {
                button_a,
                button_b,
                prize,
            })
            .parse(s)
    }

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn cramer(&self) -> u64 {
        let denom = (self.button_a.0 * self.button_b.1)
            .wrapping_sub(self.button_a.1 * self.button_b.0) as i64;
        if denom == 0 {
            return 0;
        }
        let num_a =
            (self.prize.0 * self.button_b.1).wrapping_sub(self.button_b.0 * self.prize.1) as i64;
        let a = num_a / denom;
        let num_b =
            (self.button_a.0 * self.prize.1).wrapping_sub(self.prize.0 * self.button_a.1) as i64;
        let b = num_b / denom;
        if num_a % denom != 0 || num_b % denom != 0 || a < 0 || b < 0 {
            0
        } else {
            3 * a as u64 + b as u64
        }
    }
}

#[aoc(day13, part1, Dumb)]
pub fn part1_dumb(s: &str) -> u32 {
    let iter = iterator(s, terminated(Machine::nom, opt((newline, newline))));
    iter.map(|machine| {
        let mut cheapest_win = None;
        for a in 0..=100 {
            let mut pos = (machine.button_a.0 * a, machine.button_a.1 * a);
            if pos > machine.prize {
                break;
            }
            let mut cost = 3 * a;
            if cheapest_win.is_some_and(|win| cost > win) {
                break;
            }
            for b in 0..=100 {
                if pos == machine.prize {
                    cheapest_win = Some((*cheapest_win.get_or_insert(cost)).min(cost));
                    break;
                }
                pos.0 += machine.button_b.0;
                pos.1 += machine.button_b.1;
                cost += 1;
                if pos > machine.prize || cheapest_win.is_some_and(|win| cost > win) {
                    break;
                }
            }
        }
        cheapest_win.unwrap_or(0) as u32
    })
    .sum()
}

#[aoc(day13, part1, Cramer)]
pub fn part1(s: &str) -> u64 {
    iterator(s, terminated(Machine::nom, opt((newline, newline))))
        .map(|machine| unsafe { machine.cramer() })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(s: &str) -> u64 {
    iterator(s, terminated(Machine::nom, opt((newline, newline))))
        .map(|mut machine| {
            machine.prize.0 += 10000000000000;
            machine.prize.1 += 10000000000000;
            machine
        })
        .map(|machine| unsafe { machine.cramer() })
        .sum()
}
