use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day12::Day12;

pub fn day12(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day12::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day12::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day12::parse(black_box(input))));
    group.bench_function("part1", |b| b.iter(|| black_box(challenge.clone()).part_one()));
    group.bench_function("part2", |b| b.iter(|| black_box(challenge.clone()).part_two()));
}

criterion_group!(benches, day12);
criterion_main!(benches);
