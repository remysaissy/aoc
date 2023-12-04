
pub fn day01_1() {
    let input_v1 = read_input_v1(include_str!("../inputs/day01.txt"));
    println!("day 1a: {}", input_v1.iter().sum::<u64>());
}

pub fn day01_2() {
    let input_v2 = read_input_v2(include_str!("../inputs/day01.txt"));
    println!("day 1b: {}", input_v2.iter().sum::<u64>());
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
                if slice.starts_with("one") {
                    numerics.push('1');
                } else if slice.starts_with("two") {
                    numerics.push('2');
                } else if slice.starts_with("three") {
                    numerics.push('3');
                } else if slice.starts_with("four") {
                    numerics.push('4');
                } else if slice.starts_with("five") {
                    numerics.push('5');
                } else if slice.starts_with("six") {
                    numerics.push('6');
                } else if slice.starts_with("seven") {
                    numerics.push('7');
                } else if slice.starts_with("eight") {
                    numerics.push('8');
                } else if slice.starts_with("nine") {
                    numerics.push('9');
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
