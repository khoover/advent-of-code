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

pub fn main() -> Result<()> {
    run_day(part1)
}

#[test]
fn example_part1() {
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
    assert_eq!(part1(INPUT).unwrap(), 3);
}
