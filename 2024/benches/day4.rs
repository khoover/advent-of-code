use criterion::{black_box, criterion_group, criterion_main, Criterion};

static SITE_INPUT: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
static MY_INPUT: &'static str = include_str!("../input/2024/day4.txt");

use aoc_2024::day4::{part1, part2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part1-small", |b| b.iter(|| part1(black_box(SITE_INPUT))));
    c.bench_function("part1-big", |b| b.iter(|| part1(black_box(MY_INPUT))));
    c.bench_function("part2-small", |b| b.iter(|| part2(black_box(SITE_INPUT))));
    c.bench_function("part2-big", |b| b.iter(|| part1(black_box(MY_INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
