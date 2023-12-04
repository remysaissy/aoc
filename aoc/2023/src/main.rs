use std::time::Instant;
use aoc_2023::day01::{day01_1, day01_2};
use aoc_2023::day02::{day02_1, day02_2};
use aoc_2023::day03::day03_1;

fn run_timed(name: &str, f: fn()) {
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    println!("{} elapsed: {:.2?}", name, elapsed);
}

fn main() {
    run_timed("day01_1", day01_1);
    run_timed("day01_2", day01_2);
    run_timed("day02_1", day02_1);
    run_timed("day02_2", day02_2);
    run_timed("day03_1", day03_1);
}