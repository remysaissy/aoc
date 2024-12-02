use aoc_2024::day01::Day01;
use aoc_2024::day02::Day02;
use aoc_2024::inputs::INPUTS;
use aoc_2024::Solver;

fn main() {
    println!("Day 1a: {}", Day01::from(INPUTS[0]).part1());
    println!("Day 1b: {}", Day01::from(INPUTS[0]).part2());
    println!("Day 2a: {}", Day02::from(INPUTS[1]).part1());
    println!("Day 2b: {}", Day02::from(INPUTS[1]).part2());
}
