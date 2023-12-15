use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2023::Solver;
use aoc_2023::day01::Day01;
use aoc_2023::day02::{day02_a, day02_b};
use aoc_2023::day03::{day03_a, day03_b};
use aoc_2023::day04::{day04_a, day04_b};
use aoc_2023::day05::{day05_a, day05_b};
use aoc_2023::inputs::INPUTS;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part1", |b| b.iter(|| Day01::from(black_box(INPUTS[0])).part1()));
    c.bench_function("day01 part2", |b| b.iter(|| Day01::from(black_box(INPUTS[0])).part2()));
    // c.bench_function("day02_a", |b| b.iter(|| day02_a(black_box(INPUTS[1]))));
    // c.bench_function("day02_b", |b| b.iter(|| day02_b(black_box(INPUTS[1]))));
    // c.bench_function("day03_a", |b| b.iter(|| day03_a(black_box(INPUTS[2]))));
    // c.bench_function("day03_b", |b| b.iter(|| day03_b(black_box(INPUTS[2]))));
    // c.bench_function("day04_a", |b| b.iter(|| day04_a(black_box(INPUTS[3]))));
    // c.bench_function("day04_b", |b| b.iter(|| day04_b(black_box(INPUTS[3]))));
    // c.bench_function("day05_a", |b| b.iter(|| day05_a(black_box(INPUTS[4]))));
    // c.bench_function("day05_b", |b| b.iter(|| day05_b(black_box(INPUTS[4]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);