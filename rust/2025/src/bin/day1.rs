use anyhow::Result;
use aoc_2025::run_day;

fn part1(s: &str) -> Result<u64> {
    let mut acc = 50_i64;
    let mut count = 0_u64;
    for l in s.lines() {
        let bytes = l.as_bytes();
        let coeff: i64 = match bytes[0] {
            b'R' => 1,
            b'L' => -1,
            _ => unreachable!(),
        };
        let num: i64 = unsafe { str::from_utf8_unchecked(&bytes[1..]) }.parse()?;
        acc += num * coeff;
        if acc % 100 == 0 {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(s: &str) -> Result<u64> {
    let mut acc = 50_u32 + (u32::MAX / 2).next_multiple_of(100);
    let mut count = 0_u64;
    for l in s.lines() {
        let bytes = l.as_bytes();
        let num: u32 = unsafe { str::from_utf8_unchecked(&bytes[1..]) }.parse()?;
        let old_acc = acc;
        match bytes[0] {
            b'R' => {
                acc += num;
                count +=
                    u32::abs_diff(u32::div_euclid(old_acc, 100), u32::div_euclid(acc, 100)) as u64;
            }
            b'L' => {
                acc -= num;
                count += u32::abs_diff(u32::div_ceil(old_acc, 100), u32::div_ceil(acc, 100)) as u64;
            }
            _ => unreachable!(),
        }
    }
    Ok(count)
}

pub fn main() -> Result<()> {
    run_day(part1, part2)
}

const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn example_part1() {
    assert_eq!(part1(INPUT).unwrap(), 3);
}

#[test]
fn example_part2() {
    assert_eq!(part2(INPUT).unwrap(), 6);
}
