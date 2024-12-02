use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::Solver;
use aoc_2024::day01::Day01;
use aoc_2024::inputs::INPUTS;

fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("day01 part1", |b| b.iter(|| Day01::from(black_box(INPUTS[0])).part1()));
//     c.bench_function("day01 part2", |b| b.iter(|| Day01::from(black_box(INPUTS[0])).part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);