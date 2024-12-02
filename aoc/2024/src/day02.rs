use crate::Solver;
use std::cmp::PartialEq;

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Unknown,
}

pub struct Day02<'a> {
    input: &'a str,
}

impl<'a> Day02<'a> {
    fn check_report(&self, line: &str, skip_first_error: bool) -> bool {
        let parts = line
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if Self::is_valid(&parts, None) == true {
            return true;
        }
        if skip_first_error == true {
            for idx in 0..parts.len() {
                if Self::is_valid(&parts, Some(idx)) == true {
                    return true;
                }
            }
        }
        false
    }

    fn is_valid(parts: &Vec<u64>, skip_index: Option<usize>) -> bool {
        let mut direction = Direction::Unknown;
        let mut prev_number = 0u64;
        for (i, number) in parts.iter().enumerate() {
            if skip_index.is_some() && Some(i) == skip_index {
                continue;
            }
            if (skip_index == Some(0) && i == 1) || i == 0 {
                prev_number = *number;
                continue;
            } else {
                if direction == Direction::Unknown {
                    if prev_number < *number {
                        direction = Direction::Increasing;
                    } else if prev_number > *number {
                        direction = Direction::Decreasing;
                    }
                }
                let diff = prev_number.abs_diff(*number);
                if diff < 1
                    || diff > 3
                    || (direction == Direction::Increasing && prev_number > *number)
                    || (direction == Direction::Decreasing && prev_number < *number)
                {
                    return false;
                }
                prev_number = *number;
            }
        }
        true
    }
}

impl<'a> Solver for Day02<'a> {
    fn part1(&self) -> u64 {
        self.input
            .lines()
            .filter(|line| self.check_report(line, false))
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.input
            .lines()
            .filter(|line| self.check_report(line, true))
            .count() as u64
    }
}

impl<'a> From<&'a str> for Day02<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_INPUT: &str = r#"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"#;

    const PART2_INPUT: &str = r#"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"#;

    #[test]
    fn part1() {
        let result = Day02::from(PART1_INPUT).part1();
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let result = Day02::from(PART2_INPUT).part2();
        assert_eq!(result, 4);
    }
}
