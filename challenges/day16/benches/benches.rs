use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use day16::Day16;

pub fn day16(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day16::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day16::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day16::parse(black_box(input))));
    group.bench_function("part1", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_one, BatchSize::SmallInput)
    });
    group.bench_function("part2", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_two, BatchSize::SmallInput)
    });

    group.finish();
}

criterion_group!(benches, day16);
criterion_main!(benches);
