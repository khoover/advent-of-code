use super::*;
use arrayvec::ArrayVec;

use anyhow::{Error, Result};
use nom::{
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::opt,
    error::VerboseError,
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    Finish, IResult, Parser,
};

#[aoc(day5, part1, Simd)]
fn part1_simd_wrapper(input: &str) -> u32 {
    unsafe { part1_simd(input.as_bytes()) }
}

#[aoc(day5, part2, Simd)]
fn part2_simd_wrapper(input: &str) -> u32 {
    unsafe { part2_simd(input.as_bytes()) }
}

#[aoc(day5, part1, Base)]
fn part1_base(s: &str) -> Result<u32> {
    let mut lut = [false; 100 * 100];

    let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
    let rest = terminated(
        fold_many1(
            terminated(parse_mapping, newline),
            || (),
            |(), (a, b)| {
                lut[(a as usize * 100) + b as usize] = true;
            },
        ),
        newline,
    )
    .parse(s.as_bytes())
    .finish()
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))?
    .0;

    assert!(lut.iter().copied().any(|x| x));

    fold_many1(
        terminated(separated_list1(tag(","), parse_digit_pair), opt(tag("\n"))),
        || 0_u32,
        move |acc, list| {
            debug!(list);
            for i in 0..list.len() {
                let a = list[i];
                for b in list[i..].iter().copied() {
                    if lut[(b as usize * 100) + a as usize] {
                        return acc;
                    }
                }
            }
            let mid = list[list.len() / 2] as u32;
            debug!(mid);
            acc + mid
        },
    )
    .parse_complete(rest)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

#[aoc(day5, part2, Base)]
fn part2_base(s: &str) -> Result<u32> {
    let mut lut = [false; 100 * 100];

    let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
    let rest = terminated(
        fold_many1(
            terminated(parse_mapping, newline),
            || (),
            |(), (a, b)| {
                lut[(a as usize * 100) + b as usize] = true;
            },
        ),
        newline,
    )
    .parse_complete(s.as_bytes())
    .finish()
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))?
    .0;

    assert!(lut.iter().copied().any(|x| x));

    fold_many1(
        terminated(separated_list1(tag(","), parse_digit_pair), opt(tag("\n"))),
        || 0_u32,
        move |acc, mut list| {
            debug!(list);
            let mut was_wrong = false;
            let mut i = 0;
            while i < list.len() {
                let mut a = list[i];
                let mut j = i;
                while j < list.len() {
                    let b = list[j];
                    if lut[(b as usize) * 100 + a as usize] {
                        list.swap(i, j);
                        a = b;
                        was_wrong = true;
                        j = i;
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
            debug!(was_wrong);
            if was_wrong {
                let mid = list[list.len() / 2] as u32;
                debug!(mid);
                acc + mid
            } else {
                acc
            }
        },
    )
    .parse_complete(rest)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
}

fn parse_digit_pair(input: &[u8]) -> IResult<&[u8], u8, VerboseError<&[u8]>> {
    take::<usize, &[u8], VerboseError<&[u8]>>(2_usize)
        .map_opt(|pair: &[u8]| Some(pair[0].checked_sub(b'0')? * 10 + pair[1].checked_sub(b'0')?))
        .parse(input)
}

pub fn part1(s: &str) -> u32 {
    unsafe { part1_simd(s.as_bytes()) }
}

pub fn part2(s: &str) -> Result<u32> {
    part2_base(s)
}

/// # Safety
/// Can only be called on x86 systems with certain instructions available.
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part1_simd(input: &[u8]) -> u32 {
    // Bitsets, given a, b, there's rule a|b iff a_must_before_b[a] & (1<<b) != 0
    let mut a_must_before_b = [0_u128; 100];

    let mut i = 0;

    macro_rules! parse {
        ($i:expr) => {{
            let i = $i;
            ((*input.get_unchecked(i) - b'0') * 10 + *input.get_unchecked(i + 1) - b'0')
        }};
    }

    loop {
        let b0 = *input.get_unchecked(i);
        if b0 == b'\n' {
            break;
        }
        let before = (b0 - b'0') * 10 + (*input.get_unchecked(i + 1) - b'0');
        let after = parse!(i + 3);
        a_must_before_b[before as usize] |= 1 << after;
        i += 6;
    }

    i += 1;
    let mut sum = 0;

    while i < input.len() && input.get_unchecked(i).is_ascii_digit() {
        let suffix = &input[i..];
        debug!(suffix);
        let d1 = parse!(i);
        let d2 = parse!(i + 3);
        let d3 = parse!(i + 6);
        debug!(d1);
        debug!(d2);
        debug!(d3);
        let mut median_idx = i + 3;
        i += 9;
        let mut set: u128 = 1 << d1;
        let mut good_line = a_must_before_b[d2 as usize] & set == 0;
        set |= 1 << d2;
        good_line &= a_must_before_b[d3 as usize] & set == 0;
        set |= 1 << d3;
        debug!(good_line);
        while i - 1 < input.len() && *input.get_unchecked(i - 1) != b'\n' {
            let next1 = parse!(i);
            let next2 = parse!(i + 3);
            i += 6;
            median_idx += 3;
            good_line &= a_must_before_b[next1 as usize] & set == 0;
            set |= 1 << next1;
            good_line &= a_must_before_b[next2 as usize] & set == 0;
            set |= 1 << next2;
        }

        if good_line {
            sum += parse!(median_idx) as u32;
        }
        debug!(sum);
    }

    sum
}

/// # Safety
/// Can only be called on x86 systems with certain instructions available.
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part2_simd(input: &[u8]) -> u32 {
    // Bitsets, given a, b, there's rule a|b iff a_must_before_b[a] & (1<<b) != 0
    let mut a_must_before_b = [0_u128; 100];

    let mut i = 0;

    macro_rules! parse {
        ($i:expr) => {{
            let i = $i;
            ((*input.get_unchecked(i) - b'0') * 10 + *input.get_unchecked(i + 1) - b'0')
        }};
    }

    loop {
        let b0 = *input.get_unchecked(i);
        if b0 == b'\n' {
            break;
        }
        let before = (b0 - b'0') * 10 + (*input.get_unchecked(i + 1) - b'0');
        let after = parse!(i + 3);
        a_must_before_b[before as usize] |= 1 << after;
        i += 6;
    }

    i += 1;
    let mut sum: u32 = 0;
    let mut parsed_line: ArrayVec<u8, 32> = ArrayVec::new();

    while i < input.len() && input.get_unchecked(i).is_ascii_digit() {
        let suffix = &input[i..];
        debug!(suffix);
        let d1 = parse!(i);
        let d2 = parse!(i + 3);
        let d3 = parse!(i + 6);
        parsed_line.push_unchecked(d1);
        parsed_line.push_unchecked(d2);
        parsed_line.push_unchecked(d3);
        debug!(d1);
        debug!(d2);
        debug!(d3);
        let mut median_idx = 1;
        i += 9;
        let mut set: u128 = 1 << d1;
        let mut good_line = a_must_before_b[d2 as usize] & set == 0;
        set |= 1 << d2;
        good_line &= a_must_before_b[d3 as usize] & set == 0;
        set |= 1 << d3;
        debug!(good_line);
        while i - 1 < input.len() && *input.get_unchecked(i - 1) != b'\n' {
            let next1 = parse!(i);
            let next2 = parse!(i + 3);
            i += 6;
            median_idx += 1;
            parsed_line.push_unchecked(next1);
            good_line &= a_must_before_b[next1 as usize] & set == 0;
            set |= 1 << next1;
            parsed_line.push_unchecked(next2);
            good_line &= a_must_before_b[next2 as usize] & set == 0;
            set |= 1 << next2;
        }

        if !good_line {
            let median = parsed_line.iter().copied().fold(0_u32, |acc, curr_val| {
                if (a_must_before_b[curr_val as usize] & set).count_ones() == median_idx {
                    assert!(acc == 0);
                    curr_val as u32
                } else {
                    acc
                }
            });
            sum += median;
        }
        parsed_line.clear();
        debug!(sum);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    static SITE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    const SITE_PART1_OUTPUT: u32 = 143;
    const SITE_PART2_OUTPUT: u32 = 123;

    static MY_INPUT: &str = include_str!("../input/2024/day5.txt");
    const MY_PART1_OUTPUT: u32 = 7024;
    const MY_PART2_OUTPUT: u32 = 4151;

    #[test]
    fn test_part1_base_site() {
        assert_eq!(part1_base(SITE_INPUT).unwrap(), SITE_PART1_OUTPUT);
    }

    #[test]
    fn test_part1_base_mine() {
        assert_eq!(part1_base(MY_INPUT).unwrap(), MY_PART1_OUTPUT);
    }

    #[test]
    fn test_part2_base_site() {
        assert_eq!(part2_base(SITE_INPUT).unwrap(), SITE_PART2_OUTPUT);
    }

    #[test]
    fn test_part2_base_mine() {
        assert_eq!(part2_base(MY_INPUT).unwrap(), MY_PART2_OUTPUT);
    }

    #[test]
    fn test_part1_simd_mine() {
        unsafe {
            assert_eq!(part1_simd(MY_INPUT.as_bytes()), MY_PART1_OUTPUT);
        }
    }

    #[test]
    fn test_part1_simd_site() {
        unsafe {
            assert_eq!(part1_simd(SITE_INPUT.as_bytes()), SITE_PART1_OUTPUT);
        }
    }

    #[test]
    fn test_part2_simd_mine() {
        unsafe {
            assert_eq!(part2_simd(MY_INPUT.as_bytes()), MY_PART1_OUTPUT);
        }
    }

    #[test]
    fn test_part2_simd_site() {
        unsafe {
            assert_eq!(part2_simd(SITE_INPUT.as_bytes()), SITE_PART2_OUTPUT);
        }
    }

    #[test]
    fn examine_input() {
        let longest = MY_INPUT.lines().map(|l| l.len() + 1).max().unwrap();
        println!("longest line is {longest}");

        let (first, second) = MY_INPUT.split_once("\n\n").unwrap();

        println!(
            "first half has {} bytes, or {} SIMD iters",
            first.len(),
            (first.len() + 1).div_ceil(30)
        );
        println!(
            "second half has {} bytes over {} lines",
            second.len(),
            second.lines().count()
        );
        assert!(
            second.lines().all(|line| line.len() % 2 == 0),
            "Some lines are even"
        );

        let mut lut = [false; 100 * 100];

        let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
        let rest = terminated(
            fold_many1(
                terminated(parse_mapping, newline),
                || (),
                |(), (a, b)| {
                    lut[(a as usize * 100) + b as usize] = true;
                },
            ),
            newline,
        )
        .parse(MY_INPUT.as_bytes())
        .finish()
        .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
        .unwrap()
        .0;

        fold_many1(
            terminated(separated_list1(tag(","), parse_digit_pair), opt(tag("\n"))),
            || 0_u32,
            move |acc, list| {
                debug!(list);
                let mut is_bad = false;
                for i in 0..list.len() {
                    let a = list[i];
                    for j in i..list.len() {
                        let b = list[j];
                        if lut[(b as usize * 100) + a as usize] {
                            is_bad = true;
                        }
                    }
                }
                if is_bad {
                    let neighbours_bad = list
                        .windows(2)
                        .any(|window| lut[(window[1] as usize * 100) + window[0] as usize]);
                    assert!(
                        neighbours_bad,
                        "Got an input that was bad but no neighbours were: {:?}",
                        list
                    );
                    return acc;
                }
                let mid = list[list.len() / 2] as u32;
                debug!(mid);
                acc + mid
            },
        )
        .parse_complete(rest)
        .finish()
        .map(|(_, x)| x)
        .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
        .unwrap();

        // let mut comparisons = [[false; 100]; 100];
        // for i in 0..100 {
        //     comparisons[i][i] = true;
        // }
        // let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
        // terminated(
        //     fold_many1(
        //         terminated(parse_mapping, newline),
        //         || (),
        //         |(), (a, b)| {
        //             comparisons[a as usize][b as usize] = true;
        //             ()
        //         },
        //     ),
        //     newline,
        // )
        // .parse_complete(MY_INPUT.as_bytes())
        // .finish()
        // .unwrap();

        // for k in 0..100 {
        //     for i in 0..100 {
        //         for j in 0..100 {
        //             comparisons[i][j] |= comparisons[i][k] && comparisons[k][j];
        //         }
        //     }
        // }

        // let mut violating_pairs: Vec<(usize, usize)> = Vec::new();
        // for i in 10..100 {
        //     for j in 10..100 {
        //         if i != j && !(comparisons[i][j] || comparisons[j][i]) {
        //             violating_pairs.push((i, j));
        //         }
        //     }
        // }

        // if violating_pairs.is_empty() {
        //     println!("We're cooking with gas now");
        // } else {
        //     println!(
        //         "Transitive closure won't work, {} bad pairs: {:?}", <--- this
        //         violating_pairs.len(),
        //         violating_pairs
        //     );
        // }
    }

    // #[test]
    // fn simd_intuition_testing() {
    //     unsafe {
    //         let mut arr = [0_u8; 16];
    //         for i in 1..16 {
    //             arr[i] = i as u8;
    //         }
    //         let test_bed = _mm_lddqu_si128(arr.as_ptr().cast());
    //         assert_eq!(_mm_extract_epi8::<0>(test_bed), 0);
    //         assert_eq!(_mm_extract_epi8::<10>(test_bed), 10); // Lanes on simd are little endian
    //         assert_eq!(
    //             _mm_movemask_epi8(_mm_cmpeq_epi8(
    //                 test_bed,
    //                 _mm_setr_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0) // set expects values in LE order too
    //             )),
    //             0
    //         );
    //     }
    // }

    // #[test]
    // fn test_simd_lut_load() {
    //     let s = MY_INPUT;
    //     let input = s.as_bytes();
    //     let mut lut = [[0_u8; 100]; 100];

    //     let parse_mapping = separated_pair(parse_digit_pair, tag("|"), parse_digit_pair);
    //     terminated(
    //         fold_many1(
    //             terminated(parse_mapping, newline),
    //             || (),
    //             |(), (a, b)| {
    //                 lut[b as usize][a as usize] = 0xFF;
    //                 ()
    //             },
    //         ),
    //         newline,
    //     )
    //     .parse_complete(s.as_bytes())
    //     .finish()
    //     .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))
    //     .unwrap();

    //     let mut b_before_a = [[0_u8; 100]; 100];

    //     let mut i = 0;

    //     unsafe {
    //         // Populate the two comparison maps. Using 0xFF for true because SIMD.
    //         let newline_mask = _mm_set1_epi8(b'\n' as i8);
    //         let shuffle_indices =
    //             _mm_setr_epi8(0, 1, 3, 4, 6, 7, 9, 10, 12, 13, -1, -1, -1, -1, -1, -1);
    //         let parse_base = _mm_set1_epi8(b'0' as i8);
    //         let madd_coeffs = _mm_setr_epi8(10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 0, 0, 0, 0, 0, 0);
    //         loop {
    //             let chunk1 = _mm_loadu_si128(input.as_ptr().add(i).cast());
    //             let chunk2 = _mm_loadu_si128(input.as_ptr().add(i + 15).cast());

    //             let chunk1_newline_check = _mm_cmpeq_epi8(chunk1, newline_mask);
    //             let chunk1_shuffled = _mm_shuffle_epi8(chunk1, shuffle_indices);
    //             let chunk2_newline_check = _mm_cmpeq_epi8(chunk2, newline_mask);
    //             let chunk2_shuffled = _mm_shuffle_epi8(chunk2, shuffle_indices);
    //             let chunk1_newline_mask = _mm_movemask_epi8(chunk1_newline_check) as u32;
    //             let chunk1_converted = _mm_sub_epi8(chunk1_shuffled, parse_base);
    //             let chunk2_newline_mask = _mm_movemask_epi8(chunk2_newline_check) as u32;
    //             let chunk2_converted = _mm_sub_epi8(chunk2_shuffled, parse_base);

    //             let chunk1_summed = _mm_maddubs_epi16(chunk1_converted, madd_coeffs);
    //             let chunk2_summed = _mm_maddubs_epi16(chunk2_converted, madd_coeffs);
    //             let full_newline_mask = chunk1_newline_mask | (chunk2_newline_mask << 15);
    //             let end_of_pairs =
    //                 full_newline_mask & 0b000000_100000_100000_100000_100000_10_u32.reverse_bits();

    //             let mut dst = MaybeUninit::<[u16; 13]>::uninit();
    //             _mm_storeu_si128(dst.as_mut_ptr().cast(), chunk1_summed);
    //             _mm_storeu_si128(dst.as_mut_ptr().cast::<u16>().add(5).cast(), chunk2_summed);
    //             let dst = dst.assume_init_ref();
    //             if end_of_pairs != 0 {
    //                 let end = (end_of_pairs as u32).trailing_zeros();
    //                 i += end as usize + 1;
    //                 let good_pairs = end / 6;
    //                 std::hint::assert_unchecked(good_pairs <= 5);
    //                 for j in 0..good_pairs as usize {
    //                     let before = dst[2 * j];
    //                     let after = dst[2 * j + 1];
    //                     b_before_a[after as usize][before as usize] = 0xFF;
    //                 }
    //                 break;
    //             } else {
    //                 for j in 0..5 {
    //                     let before = dst[2 * j];
    //                     let after = dst[2 * j + 1];
    //                     b_before_a[after as usize][before as usize] = 0xFF;
    //                 }
    //                 i += 30;
    //             }
    //         }
    //     }

    //     assert_eq!(b_before_a, lut);
    // }
}
