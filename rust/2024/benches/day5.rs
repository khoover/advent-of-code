use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
static MY_INPUT: &str = include_str!("../input/2024/day5.txt");

use aoc_2024::day5::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day5-part1-small", |b| {
        b.iter(|| unsafe { part1_simd(black_box(SITE_INPUT.as_bytes())) })
    });
    c.bench_function("day5-part1-big", |b| {
        b.iter(|| unsafe { part1_simd(black_box(MY_INPUT.as_bytes())) })
    });
    // c.bench_function("day5-part2-small", |b| {
    //     b.iter(|| unsafe { part2_simd(black_box(SITE_INPUT.as_bytes())) })
    // });
    // c.bench_function("day5-part2-big", |b| {
    //     b.iter(|| unsafe { part2_simd(black_box(MY_INPUT.as_bytes())) })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
