use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2023::day01::{day01_a, day01_b};
use aoc_2023::day02::{day02_a, day02_b};
use aoc_2023::day03::{day03_a, day03_b};
use aoc_2023::inputs::INPUTS;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01_a", |b| b.iter(|| day01_a(black_box(INPUTS[0]))));
    c.bench_function("day01_b", |b| b.iter(|| day01_b(black_box(INPUTS[0]))));
    c.bench_function("day02_a", |b| b.iter(|| day02_a(black_box(INPUTS[1]))));
    c.bench_function("day02_b", |b| b.iter(|| day02_b(black_box(INPUTS[1]))));
    c.bench_function("day03_a", |b| b.iter(|| day03_a(black_box(INPUTS[2]))));
    c.bench_function("day03_b", |b| b.iter(|| day03_b(black_box(INPUTS[2]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);