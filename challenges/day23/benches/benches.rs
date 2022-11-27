use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day23::Day23;

pub fn day23(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day23::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day23::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day23::parse(black_box(input))));
    group.bench_function("part1", |b| b.iter(|| challenge.part_one()));
    group.bench_function("part2", |b| b.iter(|| challenge.part_two()));

    group.finish();
}

criterion_group!(benches, day23);
criterion_main!(benches);
