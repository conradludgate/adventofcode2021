use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use day13::Day13;

pub fn day13(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day13::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day13::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day13::parse(black_box(input))));
    group.bench_function("part1", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_one, BatchSize::SmallInput)
    });
    group.bench_function("part2", |b| {
        b.iter_batched(|| challenge.clone(), Day13::part2, BatchSize::SmallInput)
    });

    group.finish();
}

criterion_group!(benches, day13);
criterion_main!(benches);
