use crate::Solver;

const WORDS_TO_DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub struct Day01<'a> {
    input: &'a str,
}

impl<'a> Solver for Day01<'a> {
    fn part1(&self) -> u64 {
        self.input.lines().map(|line| {
            let chars = line.chars();
            let numerics: Vec<char> = chars.filter(|x| x.is_numeric()).collect();
            let calibration = [
                numerics.first(),
                numerics.last(),
            ]
                .into_iter()
                .flatten().collect::<String>();
            calibration.parse().unwrap_or(0)
        }).into_iter().sum()
            //.collect::<Vec<u64>>()
    }

    fn part2(&self) -> u64 {
        self.input.lines().map(|line| {
            let mut numerics: Vec<char> = Vec::with_capacity(line.len());
            let mut idx: usize = 0;
            while idx < line.len() {
                let slice = line.get(idx..).unwrap();
                let s = slice.chars().nth(0).unwrap();
                if s.is_numeric() {
                    numerics.push(s);
                } else {
                    let mut digit = 0;
                    for word in WORDS_TO_DIGITS {
                        if slice.starts_with(word) {
                            let value = char::from_digit(digit, 10).unwrap();
                            numerics.push(value);
                            break;
                        }
                        digit += 1;
                    }
                }
                idx += 1;
            }
            let calibration = [
                numerics.first(),
                numerics.last(),
            ]
                .into_iter()
                .flatten().collect::<String>();
            calibration.parse().unwrap_or(0)
        }).into_iter().sum()
            //.collect::<Vec<u64>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Day01Err;

impl<'a> From<&'a str> for Day01<'a> {

    fn from(input: &'a str) -> Self {
        Self {
            input
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_INPUT: &str = "3six5\n3fdfd\nyhtgrfecd";
    const PART2_INPUT: &str = "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen\n\
        jhgfdsgfhjfds";

    #[test]
    fn part1() {
        let result = Day01::from(PART1_INPUT).part1();
        assert_eq!(vec![35,33,0].iter().sum::<u64>(), result);
    }

    #[test]
    fn part2() {
        let result = Day01::from(PART2_INPUT).part2();
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76, 0].iter().sum::<u64>(), result);
    }
}
