use crate::Solver;

pub struct Day01<'a> {
    input: &'a str,
}

impl<'a> Day01<'a> {
    fn build_lists(&self) -> (Vec<u64>, Vec<u64>) {
        let mut left: Vec<u64> = Vec::new();
        let mut right: Vec<u64> = Vec::new();

        for line in self.input.lines() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            let left_number: u64 = parts[0].parse().unwrap();
            let right_number: u64 = parts[1].parse().unwrap();
            left.push(left_number);
            right.push(right_number);
        }
        left.sort();
        right.sort();
        (left, right)
    }
}

impl<'a> Solver for Day01<'a> {
    fn part1(&self) -> u64 {
        let (left, right) = self.build_lists();
        let mut sum_distances: u64 = 0;
        for (i, left_number) in left.iter().enumerate() {
            let right_number = right.get(i).unwrap();
            sum_distances += left_number.abs_diff(*right_number);
        }
        sum_distances
    }

    fn part2(&self) -> u64 {
        let (left, right) = self.build_lists();
        let mut similarity_score: u64 = 0;
        for left_number in left.iter() {
            let mut count: u64 = 0;
            for right_number in right.iter() {
                if *right_number > *left_number {
                    break;
                } else if *right_number == *left_number {
                    count += 1;
                }
            }
            let similarity_inc = left_number * count;
            similarity_score += similarity_inc;
        }
        similarity_score
    }
}

impl<'a> From<&'a str> for Day01<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_INPUT: &str = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;
    const PART2_INPUT: &str = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;

    #[test]
    fn part1() {
        let result = Day01::from(PART1_INPUT).part1();
        assert_eq!(result, vec![2, 1, 0, 1, 2, 5].iter().sum::<u64>());
    }

    #[test]
    fn part2() {
        let result = Day01::from(PART2_INPUT).part2();
        assert_eq!(result, vec![9, 4, 0, 0, 9, 9].iter().sum::<u64>());
    }
}
