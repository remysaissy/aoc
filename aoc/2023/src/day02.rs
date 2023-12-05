
const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

pub fn day02_a(input: &str) -> u64 {
    let result = read_input_v1(input);
    result.iter().sum::<u64>()
}

pub fn day02_b(input: &str) -> u64 {
    let result = read_input_v2(input);
    result.iter().sum::<u64>()
}

fn read_input_v1(input: &str) -> Vec<u64> {
    input.lines().map(|line| {
        let (game_label, sets) = line.trim().split_once(':').unwrap();
        let game_id = game_label.split_once(" ").unwrap().1.parse::<u64>().unwrap();
        let is_possible = sets.trim().split(';').all(|x| {
            x.split(',').all(|x| {
            let (count, color) = x.trim().split_once(' ').unwrap();
            match color.trim() {
                "red" => count.parse::<u64>().unwrap() <= MAX_RED,
                "green" => count.parse::<u64>().unwrap() <= MAX_GREEN,
                "blue" => count.parse::<u64>().unwrap() <= MAX_BLUE,
                _ => false
            }
            })
        });
        if is_possible {
            Some(game_id)
        } else {
            None
        }
    }).flatten().collect::<Vec<u64>>()
}

fn read_input_v2(input: &str) -> Vec<u64> {
    input.lines().map(|line| {
        let sets = line.trim().split_once(':').unwrap().1;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in sets.trim().split(';').into_iter() {
            for pair in set.split(',').into_iter() {
                let (_count, _color) = pair.trim().split_once(' ').unwrap();
                let count = _count.parse::<u64>().unwrap();
                let color = _color.trim();
                if color == "red" && count > min_red {
                    min_red = count;
                }
                if color == "green" && count > min_green {
                    min_green = count;
                }
                if color == "blue" && count > min_blue {
                    min_blue = count;
                }
            }
        }
        min_red * min_green * min_blue
    }).collect::<Vec<u64>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_v1() {
        let input = read_input_v1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(vec![1,2,5], input);
    }

    #[test]
    fn test_input_v2() {
        let input = read_input_v2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(vec![48,12,1560,630,36], input);
    }
}
