use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day10::Day10;

pub fn day10(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day10::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day10::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day10::parse(black_box(input))));
    group.bench_function("part1", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_one, BatchSize::SmallInput)
    });
    group.bench_function("part2", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_two, BatchSize::SmallInput)
    });

    group.finish();
}

criterion_group!(benches, day10);
criterion_main!(benches);
