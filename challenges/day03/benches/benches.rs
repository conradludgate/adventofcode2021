use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day03::Day03;

pub fn day03(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day03::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day03::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day03::parse(black_box(input))));
    group.bench_function("part1", |b| b.iter(|| black_box(challenge.clone()).part_one()));
    group.bench_function("part2", |b| b.iter(|| black_box(challenge.clone()).part_two()));
}

criterion_group!(benches, day03);
criterion_main!(benches);
