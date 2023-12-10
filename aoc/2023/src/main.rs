use aoc_2023::day01::{day01_a, day01_b};
use aoc_2023::day02::{day02_a, day02_b};
use aoc_2023::day03::{day03_a, day03_b};
use aoc_2023::day04::{day04_a, day04_b};
use aoc_2023::inputs::INPUTS;

fn main() {
    println!("Day 1a: {}", day01_a(INPUTS[0]));
    println!("Day 1b: {}", day01_b(INPUTS[0]));
    println!("Day 2a: {}", day02_a(INPUTS[1]));
    println!("Day 2b: {}", day02_b(INPUTS[1]));
    println!("Day 3a: {}", day03_a(INPUTS[2]));
    println!("Day 3b: {}", day03_b(INPUTS[2]));
    println!("Day 4a: {}", day04_a(INPUTS[3]));
    println!("Day 4b: {}", day04_b(INPUTS[3]));
}