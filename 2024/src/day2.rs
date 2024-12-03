use std::hint::unreachable_unchecked;

use crate::utils::*;
use arrayvec::ArrayVec;
use coz::scope;
use nom::{
    character::complete::{i16, space0, space1},
    combinator::iterator,
    error::VerboseError,
    multi::separated_list1,
    sequence::terminated,
};

#[aoc(day2, part1, Naive)]
pub fn part1_naive(input: &str) -> usize {
    scope!("part1");
    input
        .lines()
        .filter(|&line| {
            let mut it = iterator(line, terminated(i16::<&str, VerboseError<&str>>, space0));
            let first = it.next().unwrap();
            let mut diffs = it.scan(first, |prev_value, next_value| {
                let res = Some(next_value - *prev_value);
                *prev_value = next_value;
                res
            });
            let first_diff = diffs.next().unwrap();
            if !(1..=3).contains(&first_diff.abs()) {
                return false;
            }
            let sign = first_diff.signum();
            diffs.all(|diff| diff.signum() == sign && (1..=3).contains(&diff.abs()))
        })
        .count()
}

#[aoc(day2, part1, Opt)]
pub fn part1_opt(input: &str) -> usize {
    scope!("part1");
    input
        .lines()
        .filter(|&line| {
            let bytes = line.as_bytes();
            let mut idx = 0;
            let mut it = std::iter::from_fn(|| {
                if idx >= bytes.len() {
                    return None;
                }
                let mut acc = bytes[idx] - b'0';
                if idx + 1 < bytes.len() && bytes[idx + 1].is_ascii_digit() {
                    acc = acc * 10 + bytes[idx + 1] - b'0';
                    idx += 3;
                } else {
                    idx += 2;
                }
                Some(acc as i16)
            });
            let first = unsafe { it.next().unwrap_unchecked() };
            let mut diffs = it.scan(first, |prev_value, next_value| {
                let res = Some(next_value - *prev_value);
                *prev_value = next_value;
                res
            });
            let first_diff = unsafe { diffs.next().unwrap_unchecked() };
            if !(1..=3).contains(&first_diff.abs()) {
                return false;
            }
            let sign = first_diff as u16 & 0x8000;
            diffs.all(|diff| (diff as u16 & 0x8000) == sign && (1..=3).contains(&diff.abs()))
        })
        .count()
}

#[aoc(day2, part1, Hyperopt)]
pub fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut bytes = input.as_bytes();
    while bytes.len() > 1 {
        let (good_line, new_bytes) = check_line(bytes);
        count += good_line as usize;
        bytes = new_bytes;
    }
    count
}

fn check_line(input: &[u8]) -> (bool, &[u8]) {
    let mut idx = 0;
    let mut stop_iter = false;
    let mut it = std::iter::from_fn(|| {
        if stop_iter {
            return None;
        }
        let mut acc = input[idx] - b'0';
        idx += 1;
        if idx == input.len() {
            stop_iter = true;
            return Some(acc as i16);
        } else {
            match input[idx] {
                b'\n' => {
                    stop_iter = true;
                    idx += 1;
                }
                b' ' => {
                    idx += 1;
                }
                b @ b'0'..=b'9' => {
                    acc = acc * 10 + (b - b'0');
                    idx += 1;
                    match input.get(idx) {
                        None => {
                            stop_iter = true;
                        }
                        Some(b'\n') => {
                            stop_iter = true;
                            idx += 1;
                        }
                        Some(b' ') => {
                            idx += 1;
                        }
                        _ => unsafe { unreachable_unchecked() },
                    }
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }
        Some(acc as i16)
    });
    let first = unsafe { it.next().unwrap_unchecked() };
    let mut diffs = it.scan(first, |prev_value, next_value| {
        let res = Some(next_value - *prev_value);
        *prev_value = next_value;
        res
    });
    let Some(first_diff) = diffs.next() else {
        return (true, &input[idx.min(input.len())..]);
    };
    let sign = first_diff as u16 & 0x8000;
    let res = diffs.fold(true, |good, diff| {
        good && (diff as u16 & 0x8000) == sign && (1..=3).contains(&diff.abs())
    }) && ((1..=3).contains(&first_diff.abs()));
    (res, &input[idx.min(input.len())..])
}

#[aoc(day2, part2, Naive)]
pub fn part2_naive(input: &str) -> usize {
    scope!("part2");
    input.lines().filter(|&line| naive_filter_fn(line)).count()
}

fn naive_filter_fn(line: &str) -> bool {
    let values = run_parse(line, separated_list1(space1, i16)).unwrap();
    for i in 0..values.len() {
        let mut good_values = values.clone();
        good_values.remove(i);
        let diffs: Vec<_> = good_values.windows(2).map(|w| w[1] - w[0]).collect();
        let increasing = diffs.iter().copied().all(|x| x > 0);
        let decreasing = diffs.iter().copied().all(|x| x < 0);
        let in_range = diffs.iter().copied().all(|x| x.abs() <= 3);
        if (increasing || decreasing) && in_range {
            return true;
        }
    }
    false
}

#[aoc(day2, part2, Opt)]
pub fn part2_opt(input: &str) -> usize {
    scope!("part2");
    input.lines().filter(|&line| opt_filter_fn(line)).count()
}

fn opt_filter_fn(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut idx = 0;
    let mut it = std::iter::from_fn(|| {
        if idx >= bytes.len() {
            return None;
        }
        let mut acc = bytes[idx] - b'0';
        if idx + 1 < bytes.len() && bytes[idx + 1].is_ascii_digit() {
            acc = acc * 10 + bytes[idx + 1] - b'0';
            idx += 3;
        } else {
            idx += 2;
        }
        Some(acc as i16)
    });
    let Some(first) = it.next() else {
        return true;
    };
    let mut diffs = it
        .scan(first, |prev_value, next_value| {
            let res = Some(next_value - *prev_value);
            *prev_value = next_value;
            Some(res)
        })
        .chain(Some(None));
    let Some(first_diff) = diffs.next() else {
        return true;
    };
    let Some(second_diff) = diffs.next() else {
        return true; // If there's no second one, then there was only one input element, after accounting for the sentinel.
    };

    // The idea is there's 4 scenarios:
    // - drop the first element
    // - drop an inner element
    // - drop the last element
    // - keep everything, but it's a trivial one
    // We fork once at the start into 1 and 4, then proceed checking.
    // We'll look at windows of 3 diffs, d1, d2, d3 (covering original inputs i1, i2, i3, i4).
    // If d1 or d2 is illegal, or they can't be paired, then either we're in scenarios 1-3 to start and thus fail,
    // or we're in scenario 4 and we branch to 2 or 3.
    // Why do we need d3? Because it could be i2 or i3 causing the issue.
    // So we split the difference and try deleting both.

    let initial_states = {
        let mut v: ArrayVec<Part2State, 3> = ArrayVec::new();

        let Some(second_inner) = second_diff else {
            return true;
        };

        // run the first step of iteration manually, because I don't want to put diffs in another chain
        // to re-insert second_diff.
        Part2State {
            prev: None,
            curr: first_diff,
            can_drop: true,
        }
        .process(second_diff, &mut v);

        if (1..=3).contains(&second_inner.abs()) {
            // second diff has to be in range, since we're assuming first is dropped, i.e. no more combos.
            unsafe {
                v.push_unchecked(Part2State {
                    prev: None,
                    curr: second_diff,
                    can_drop: false,
                })
            };
        }

        v
    };

    // println!("{initial_states:?}");

    let final_states = diffs.fold(initial_states, |states, next| {
        let mut res = ArrayVec::new();
        states
            .into_iter()
            .for_each(|state| state.process(next, &mut res));
        // println!("{next:?} => {res:?}");
        res
    });

    !final_states.is_empty()
}

/// # State invariant
/// prev will always pass check_legal_range
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Part2State {
    prev: Option<i16>,
    curr: Option<i16>,
    can_drop: bool,
}

impl Part2State {
    #[inline(always)]
    fn process(self, next: Option<i16>, out: &mut ArrayVec<Part2State, 3>) {
        let Self {
            prev,
            curr,
            can_drop,
        } = self;
        if check_diff_pair(prev, curr) && check_legal_range(curr) {
            unsafe {
                out.push_unchecked(Part2State {
                    prev: curr,
                    curr: next,
                    can_drop,
                })
            };
        } else if can_drop {
            Self::cold_process(prev, curr, next, out);
        }
    }

    #[cold]
    #[inline(never)]
    fn cold_process(
        prev: Option<i16>,
        curr: Option<i16>,
        next: Option<i16>,
        out: &mut ArrayVec<Part2State, 3>,
    ) {
        let prev_curr_combo = combine_diffs(prev, curr);
        if check_diff_pair(prev_curr_combo, next)
            && check_legal_range(prev_curr_combo)
            && check_legal_range(next)
        {
            unsafe {
                out.push_unchecked(Part2State {
                    prev: prev_curr_combo,
                    curr: next,
                    can_drop: false,
                })
            };
        }
        let curr_next_combo = combine_diffs(curr, next);
        if check_diff_pair(prev, curr_next_combo) && check_legal_range(curr_next_combo) {
            unsafe {
                out.push_unchecked(Part2State {
                    prev,
                    curr: curr_next_combo,
                    can_drop: false,
                })
            };
        }
    }
}

fn combine_diffs(a: Option<i16>, b: Option<i16>) -> Option<i16> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}

/// Checks if a pair of diffs (real diffs or sentinels) are same sign.
/// Sentinels get a free pass, since either it's the final sentinel as next (in which
/// case we check prev and curr)
fn check_diff_pair(a: Option<i16>, b: Option<i16>) -> bool {
    a.zip(b)
        .is_none_or(|(x, y)| ((x as u16) ^ (y as u16)) & 0x8000 == 0)
}

fn check_legal_range(x: Option<i16>) -> bool {
    x.is_none_or(|x| (1..=3).contains(&x.abs()))
}

#[aoc(day2, part2, Hyperopt)]
pub fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut bytes = input.as_bytes();
    while bytes.len() > 1 {
        let (good_line, new_bytes) = check_line_part2(bytes);
        count += good_line as usize;
        bytes = new_bytes;
    }
    count
}

#[inline(always)]
fn check_line_part2(input: &[u8]) -> (bool, &[u8]) {
    let mut idx = 0;
    let mut stop_iter = false;
    let mut it = std::iter::from_fn(|| {
        if stop_iter {
            return None;
        }
        let mut acc = input[idx] - b'0';
        idx += 1;
        if idx == input.len() {
            stop_iter = true;
            return Some(acc as i16);
        } else {
            match input[idx] {
                b'\n' => {
                    stop_iter = true;
                    idx += 1;
                }
                b' ' => {
                    idx += 1;
                }
                b if b.is_ascii_digit() => {
                    acc = acc * 10 + (b - b'0');
                    idx += 1;
                    match input.get(idx) {
                        None => {
                            stop_iter = true;
                        }
                        Some(b'\n') => {
                            stop_iter = true;
                            idx += 1;
                        }
                        Some(b' ') => {
                            idx += 1;
                        }
                        _ => unsafe { unreachable_unchecked() },
                    }
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }
        Some(acc as i16)
    });
    let first = unsafe { it.next().unwrap_unchecked() };
    let mut diffs = it
        .scan(first, |prev_value, next_value| {
            let res = Some(next_value - *prev_value);
            *prev_value = next_value;
            Some(res)
        })
        .chain(Some(None));
    let first_diff = unsafe { diffs.next().unwrap_unchecked() };
    let second_diff = unsafe { diffs.next().unwrap_unchecked() };

    // The idea is there's 4 scenarios:
    // - drop the first element
    // - drop an inner element
    // - drop the last element
    // - keep everything, but it's a trivial one
    // We fork once at the start into 1 and 4, then proceed checking.
    // We'll look at windows of 3 diffs, d1, d2, d3 (covering original inputs i1, i2, i3, i4).
    // If d1 or d2 is illegal, or they can't be paired, then either we're in scenarios 1-3 to start and thus fail,
    // or we're in scenario 4 and we branch to 2 or 3.
    // Why do we need d3? Because it could be i2 or i3 causing the issue.
    // So we split the difference and try deleting both.

    let initial_states = {
        let mut v: ArrayVec<Part2State, 3> = ArrayVec::new();

        let second_inner = unsafe { second_diff.unwrap_unchecked() };

        // run the first step of iteration manually, because I don't want to put diffs in another chain
        // to re-insert second_diff.
        Part2State {
            prev: None,
            curr: first_diff,
            can_drop: true,
        }
        .process(second_diff, &mut v);

        if (1..=3).contains(&second_inner.abs()) {
            // second diff has to be in range, since we're assuming first is dropped, i.e. no more combos.
            v.push(Part2State {
                prev: None,
                curr: second_diff,
                can_drop: false,
            });
        }

        v
    };

    // println!("{initial_states:?}");

    let final_states = diffs.fold(initial_states, |states, next| {
        let mut res = ArrayVec::new();
        states
            .into_iter()
            .for_each(|state| state.process(next, &mut res));
        // println!("{next:?} => {res:?}");
        res
    });

    (!final_states.is_empty(), &input[idx.min(input.len())..])
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    static INPUT: &'static str = include_str!("../input/2024/day2.txt");

    #[test]
    fn test_part1_hyper() {
        for (offset, expected) in (0..6)
            .zip([true, false, false, false, false, true])
            .map(|(x, b)| (x * 10, b))
        {
            let (actual, remainder) = check_line(&EXAMPLE.as_bytes()[offset..]);
            assert_eq!(actual, expected);
            assert_eq!(
                remainder,
                &EXAMPLE.as_bytes()[EXAMPLE.as_bytes().len().min(offset + 10)..]
            )
        }
    }

    #[test]
    fn test_part2_hyper() {
        assert_eq!(part2_hyper(EXAMPLE), 4);
    }

    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(EXAMPLE), 4);
    }

    #[test]
    fn test_part2_opt() {
        assert_eq!(part2_opt(EXAMPLE), 4);
    }

    #[test]
    fn test_part2_opt_by_line() {
        INPUT.lines().enumerate().for_each(|(idx, line)| {
            let expected = naive_filter_fn(line);
            let actual = opt_filter_fn(line);
            assert_eq!(
                expected, actual,
                "Naive and opt disagree on line {idx}: {line}"
            );
            println!("Done line {idx}");
        });
    }
}
