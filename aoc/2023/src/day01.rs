pub fn day01_a(input: &str) -> u64 {
    let result = read_input_v1(input);
    result.iter().sum::<u64>()
}

pub fn day01_b(input: &str) -> u64 {
    let result = read_input_v2(input);
    result.iter().sum::<u64>()
}

fn read_input_v1(input: &str) -> Vec<u64> {
    input.lines().map(|line| {
        let chars = line.chars();
        let numerics: Vec<char> = chars.filter(|x| x.is_numeric()).collect();
        let calibration = [
        numerics.first(),
        numerics.last(),
    ]
    .into_iter()
    .flatten().collect::<String>();
    calibration.parse().unwrap_or(0)
    }).collect::<Vec<u64>>()
}

const WORDS_TO_DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn read_input_v2(input: &str) -> Vec<u64> {
    input.lines().map(|line| {
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
    }).collect::<Vec<u64>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_v1() {
        let input = read_input_v1("3six5\n3fdfd\nyhtgrfecd");
        assert_eq!(vec![35,33,0], input);
    }

    #[test]
    fn test_input_v2() {
        let input = read_input_v2("two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen\n\
        jhgfdsgfhjfds");

        println!("{:?}", input);
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76, 0], input);
    }
}
