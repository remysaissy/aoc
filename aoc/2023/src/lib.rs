pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod inputs;

pub trait Solver {

    fn part1(&self) -> u64;

    fn part2(&self) -> u64;
}