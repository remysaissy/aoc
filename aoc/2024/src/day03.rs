use crate::Solver;
use regex::Regex;
use std::cmp::PartialEq;

pub struct Day03<'a> {
    input: &'a str,
    re: Regex,
    re2: Regex,
}

#[derive(PartialEq)]
pub enum InstType {
    Do,
    Dont,
}

impl<'a> Day03<'a> {
    fn run_mul(&self, slice: &str) -> u64 {
        let mut sum = 0u64;
        for (_, [left, right]) in self.re.captures_iter(slice).map(|c| c.extract()) {
            let left_number = left.parse::<u64>().unwrap();
            let right_number = right.parse::<u64>().unwrap();
            sum += left_number * right_number;
        }
        sum
    }

    fn consume_until(input: &mut &[u8], target: &[u8]) -> Option<&[u8]> {
        if target.is_empty() || input.is_empty() {
            return None;
        }
        for i in 0..=input.len().saturating_sub(target.len()) {
            if &input[i..i + target.len()] == target {
                let consumed = &input[..i + target.len()];
                *input = &input[i + target.len()..];
                return Some(consumed);
            }
        }
        None
    }
}

impl<'a> Solver for Day03<'a> {
    fn part1(&self) -> u64 {
        let mut sum = 0u64;
        for line in self.input.lines() {
            sum += self.run_mul(line);
        }
        sum
    }

    fn part2(&self) -> u64 {
        let mut sum = 0u64;
        let mut enabled = true;
        for capture in self.re2.captures_iter(self.input) {
            if capture.name("dont") != None {
                enabled = false;
            } else if capture.name("do") != None {
                enabled = true;
            } else if enabled {
                let left_number = capture.name("a").unwrap().as_str().parse::<u64>().unwrap();
                let right_number = capture.name("b").unwrap().as_str().parse::<u64>().unwrap();
                sum += left_number * right_number;
            }
        }
        sum
    }
}

impl<'a> From<&'a str> for Day03<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            input,
            re: Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap(),
            re2: Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<a>\d+),(?<b>\d+)\))")
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const PART2_INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn part1() {
        let result = Day03::from(PART1_INPUT).part1();
        assert_eq!(result, 161);
    }

    #[test]
    fn part2() {
        let result = Day03::from(PART2_INPUT).part2();
        assert_eq!(result, 48);
    }
}
