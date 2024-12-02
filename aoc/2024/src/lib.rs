pub mod day01;
pub mod inputs;
pub mod day02;

pub trait Solver {

    fn part1(&self) -> u64;

    fn part2(&self) -> u64;
}