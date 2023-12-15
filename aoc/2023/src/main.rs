use aoc_2023::day01::Day01;
use aoc_2023::day02::{day02_a, day02_b};
use aoc_2023::day03::{day03_a, day03_b};
use aoc_2023::day04::{day04_a, day04_b};
use aoc_2023::day05::{day05_a, day05_b};
use aoc_2023::inputs::INPUTS;
use aoc_2023::Solver;

fn main() {
    println!("Day 1a: {}", Day01::from(INPUTS[0]).part1());
    println!("Day 1b: {}", Day01::from(INPUTS[0]).part2());
    println!("Day 2a: {}", day02_a(INPUTS[1]));
    println!("Day 2b: {}", day02_b(INPUTS[1]));
    println!("Day 3a: {}", day03_a(INPUTS[2]));
    println!("Day 3b: {}", day03_b(INPUTS[2]));
    println!("Day 4a: {}", day04_a(INPUTS[3]));
    println!("Day 4b: {}", day04_b(INPUTS[3]));
    println!("Day 5a: {}", day05_a(INPUTS[4]));
    println!("Day 5b: {}", day05_b(INPUTS[4]));
}