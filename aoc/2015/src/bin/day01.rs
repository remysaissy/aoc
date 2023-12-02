
fn main() {
    let input_v1 = read_input_v1(include_str!("../../inputs/day01.txt"));
    println!("day 1a: {}", input_v1.iter().sum::<i64>());

    let input_v2 = read_input_v2(include_str!("../../inputs/day01.txt"));
    println!("day 1b: {}", input_v2);
}

fn read_input_v1(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    })).flatten().collect::<Vec<i64>>()
}

fn read_input_v2(input: &str) -> i64 {
    let positions = read_input_v1(input);
    let mut floor = 0;
    for (pos, e) in positions.iter().enumerate() {
        floor += *e;
        if floor == -1 {
            return (pos + 1) as i64;
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_v1() {
        let input = read_input_v1("(())");
        assert_eq!(0, input.iter().sum::<i64>());

        let input = read_input_v1("(()(()(");
        assert_eq!(3, input.iter().sum::<i64>());

        let input = read_input_v1(")())())");
        assert_eq!(-3, input.iter().sum::<i64>());
    }

    #[test]
    fn test_input_v2() {
        let input = read_input_v2(")");
        assert_eq!(1, input);

        let input = read_input_v2("()())");
        assert_eq!(5, input);
    }
}
