use criterion::{black_box, criterion_group, criterion_main, Criterion};

static SITE_INPUT: &'static str = "47|53
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
static MY_INPUT: &'static str = include_str!("../input/2024/day5.txt");

use aoc_2024::day5::{part1, part2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part1-small", |b| b.iter(|| part1(black_box(SITE_INPUT))));
    c.bench_function("part1-big", |b| b.iter(|| part1(black_box(MY_INPUT))));
    c.bench_function("part2-small", |b| b.iter(|| part2(black_box(SITE_INPUT))));
    c.bench_function("part2-big", |b| b.iter(|| part2(black_box(MY_INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
