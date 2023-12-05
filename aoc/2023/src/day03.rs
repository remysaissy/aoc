
pub fn day03_a(input: &str) -> u64 {
    let result = read_input_v1(input);
    result.iter().sum::<u64>()
}

pub fn day03_b(input: &str) -> u64 {
    let result = read_input_v2(input);
    result.iter().fold(1, |acc, e| acc * e)
}

fn read_input_v1(input: &str) -> Vec<u64> {
    let mut lines: Vec<Option<&str>> = input.lines().map(|line| Some(line)).collect();
    lines.insert(0, None);
    lines.insert(lines.len(), None);

    let mut found_number = false;
    let mut start_index: Option<usize> = None;
    let mut end_index: Option<usize> = None;
    lines.windows(3).flat_map(|window| {
        let (prev_line, cur_line, next_line) = (window[0], window[1], window[2]);
        let line_len = cur_line.map_or(0, |cur_line| cur_line.len());
        cur_line.map(|cur_line| {
            cur_line.chars().enumerate().filter_map(|(idx, c)| {
                let mut result = None;
                let is_digit = c.is_digit(10);
                if start_index.is_none() {
                    if !is_digit {
                        return None;
                    } else {
                        start_index = Some(idx);
                    }
                }
                if start_index.is_some() {
                    if is_digit {
                        if prev_line.is_some_and(|prev_line|
                            check_valid_gear_v1(prev_line.chars().nth(idx))
                                || (idx > 0 && check_valid_gear_v1(prev_line.chars().nth(idx - 1)))
                                || (idx + 1 < line_len && check_valid_gear_v1(prev_line.chars().nth(idx + 1))))
                            || next_line.is_some_and(|next_line|
                            check_valid_gear_v1(next_line.chars().nth(idx))
                                || (idx > 0 && check_valid_gear_v1(next_line.chars().nth(idx - 1)))
                                || (idx + 1 < line_len && check_valid_gear_v1(next_line.chars().nth(idx + 1))))
                            || (idx > 0 && check_valid_gear_v1(cur_line.chars().nth(idx - 1)))
                            || (idx + 1 < line_len && check_valid_gear_v1(cur_line.chars().nth(idx + 1))) {
                            found_number = true;
                        }
                        if idx + 1 == line_len {
                            end_index = Some(idx + 1);
                        }
                    } else {
                        end_index = Some(idx);
                    }
                    if end_index.is_some() {
                        if found_number {
                            let s = &cur_line[start_index.unwrap()..end_index.unwrap()];
                            let number = s.parse::<u64>().unwrap();
                            result = Some(number);
                        }
                        start_index = None;
                        end_index = None;
                        found_number = false;
                    }
                }
                result
            })
        }).into_iter().flatten().collect::<Vec<u64>>()
    }).collect::<Vec<u64>>()
}

fn check_valid_gear_v1(c: Option<char>) -> bool {
    c.is_some_and(|c| !c.is_digit(10) && c != '.')
}

fn read_input_v2(input: &str) -> Vec<u64> {
    let mut lines: Vec<Option<&str>> = input.lines().map(|line| Some(line)).collect();
    lines.insert(0, None);
    lines.insert(lines.len(), None);

    lines.windows(3).flat_map(|window| {
        let (prev_line, cur_line, next_line) = (window[0], window[1], window[2]);
        let line_len = cur_line.map_or(0, |cur_line| cur_line.len());
        cur_line.map(|cur_line| {
            cur_line.chars().enumerate().filter_map(|(idx, c)| {
                let mut result = None;
                if c != '*' {
                    return None;
                }
                return None;
            })
        }).into_iter().flatten().collect::<Vec<u64>>()
    }).collect::<Vec<u64>>()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n\
.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn test_1() {
        let input = read_input_v1(TEST_INPUT);
        assert_eq!(vec![467,35,633,617,592,755,664,598], input);
    }

    #[test]
    fn test_2() {
        let input = read_input_v2(TEST_INPUT);
        assert_eq!(vec![467,35], input);
    }
}
