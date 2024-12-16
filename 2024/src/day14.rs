use std::cmp::Ordering;

use super::*;
use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{i16, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Robot {
    position: (i16, i16),
    velocity: (i16, i16),
}

#[aoc_generator(day14)]
fn day14_gen(s: &str) -> Result<Vec<Robot>> {
    run_parse(
        s,
        separated_list1(
            newline,
            separated_pair(
                preceded(tag("p="), separated_pair(i16, tag(","), i16)),
                space1,
                preceded(tag("v="), separated_pair(i16, tag(","), i16)),
            )
            .map(|(position, velocity)| Robot { position, velocity }),
        ),
    )
}

#[aoc(day14, part1)]
pub fn part1(robots: &[Robot]) -> u32 {
    const WIDTH: i16 = 101;
    const HEIGHT: i16 = 103;
    const WIDTH_MID: i16 = 50;
    const HEIGHT_MID: i16 = 51;

    let mut quadrants = [[0_u32; 2]; 2];

    robots.iter().for_each(|robot| {
        let final_x = (robot.position.0
            + robot
                .velocity
                .0
                .checked_mul(100)
                .map(|v| v.rem_euclid(WIDTH))
                .unwrap())
        .rem_euclid(WIDTH);
        let final_y = (robot.position.1
            + robot
                .velocity
                .1
                .checked_mul(100)
                .map(|v| v.rem_euclid(HEIGHT))
                .unwrap())
        .rem_euclid(HEIGHT);
        let x_ord = final_x.cmp(&WIDTH_MID);
        let y_ord = final_y.cmp(&HEIGHT_MID);
        if x_ord != Ordering::Equal && y_ord != Ordering::Equal {
            quadrants[x_ord.is_gt() as usize][y_ord.is_gt() as usize] += 1;
        }
    });

    quadrants.into_iter().flatten().product()
}

#[aoc(day14, part2)]
pub fn part2(robots: &[Robot]) -> u32 {
    let mut robots: Vec<Robot> = robots.into();
    const WIDTH: i16 = 101;
    const HEIGHT: i16 = 103;
    let mut seconds = 0;

    for i in 1..WIDTH * HEIGHT + 1 {
        let mut occupied = [[false; HEIGHT as usize]; WIDTH as usize];
        robots.iter_mut().for_each(|robot| {
            robot.position.0 = (robot.position.0 + robot.velocity.0).rem_euclid(WIDTH);
            robot.position.1 = (robot.position.1 + robot.velocity.1).rem_euclid(HEIGHT);
            occupied[robot.position.0 as usize][robot.position.1 as usize] = true;
        });
        if (0..WIDTH as usize - 16)
            .into_par_iter()
            .any(|x| (0..HEIGHT as usize).any(|y| (0..16).all(|k| occupied[x + k][y])))
        {
            seconds = i as u32;
            break;
        }
    }

    seconds
}

#[cfg(test)]
mod test {
    use super::*;

    static SITE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let robots = day14_gen(SITE_INPUT).unwrap();
        const WIDTH: i16 = 11;
        const HEIGHT: i16 = 7;
        const WIDTH_MID: i16 = 5;
        const HEIGHT_MID: i16 = 3;

        let mut quadrants = [[0_u32; 2]; 2];

        robots.iter().for_each(|robot| {
            let final_x = (robot.position.0
                + robot
                    .velocity
                    .0
                    .checked_mul(100)
                    .map(|v| v.rem_euclid(WIDTH))
                    .unwrap())
            .rem_euclid(WIDTH);
            let final_y = (robot.position.1
                + robot
                    .velocity
                    .1
                    .checked_mul(100)
                    .map(|v| v.rem_euclid(HEIGHT))
                    .unwrap())
            .rem_euclid(HEIGHT);
            let final_pos = (final_x, final_y);
            debug!(final_pos);
            let x_ord = final_x.cmp(&WIDTH_MID);
            let y_ord = final_y.cmp(&HEIGHT_MID);
            if x_ord != Ordering::Equal && y_ord != Ordering::Equal {
                quadrants[x_ord.is_gt() as usize][y_ord.is_gt() as usize] += 1;
            }
        });

        debug!(quadrants);
        assert_eq!(quadrants.into_iter().flatten().product::<u32>(), 12_u32);
    }
}
