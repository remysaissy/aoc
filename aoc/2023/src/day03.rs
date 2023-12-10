use std::ops::Range;
use std::usize;

pub fn day03_a(input: &str) -> u64 {
    let schematic = Schematic::new(input);
    let result = schematic.solve_1();
    result.iter().sum::<u64>()
}

pub fn day03_b(_input: &str) -> u64 {
    // let result = read_input_v2(input);
    // result.iter().fold(1, |acc, e| acc * e)
    42
}

struct Number {
    value: u64,
    line_no: usize,
    prev_adjacent: Option<Range<usize>>,
    next_adjacent: Option<Range<usize>>,
    prev: Option<usize>,
    next: Option<usize>,
}

struct Schematic {
    schema: String,
    numbers: Vec<Number>,
}

impl Schematic {
    fn new(input: &str) -> Self {
        Self {
            schema: input.to_string(),
            numbers: Schematic::collect_numbers(input),
        }
    }

    fn collect_numbers(input: &str) -> Vec<Number> {
        let mut start_index: Option<usize> = None;
        let mut end_index: Option<usize> = None;
        let mut numbers: Vec<Number> = Vec::new();
        let total_lines = input.lines().count();
        for (line_no, line) in input.lines().enumerate() {
            let line_len = line.len();
            for (line_offset, c) in line.chars().enumerate() {
                let is_digit = c.is_digit(10);
                if is_digit {
                    if start_index.is_none() {
                        start_index = Some(line_offset);
                    }
                    end_index = Some(line_offset);
                }
                if start_index.is_some() {
                    if !is_digit || line_offset+1 == line_len {
                        let start = start_index.unwrap();
                        let end = end_index.unwrap();
                        let number_slice = &line[start..=end];
                        let value = number_slice.parse::<u64>().unwrap();
                        let prev: Option<usize> = if start > 0 { Some(start-1) } else { None };
                        let next: Option<usize> = if end+1 < line_len { Some(end+1) } else { None };
                        let prev_adjacent = if line_no > 0 { Some(Range {start: prev.unwrap_or(start), end: next.unwrap_or(end)}) } else { None };
                        let next_adjacent = if line_no+1 < total_lines { Some(Range {start: prev.unwrap_or(start), end: next.unwrap_or(end)}) } else { None };
                        numbers.push(Number {
                            value,
                            line_no,
                            prev_adjacent,
                            next_adjacent,
                            prev,
                            next,
                        });
                        start_index = None;
                        end_index = None;
                    }
                }
            }
        }
        numbers
    }

    fn solve_1(&self) -> Vec<u64> {
        self.numbers.iter().map(|number| {
            if number.prev.is_some_and(|c| self.schema.lines().nth(number.line_no).unwrap().chars().nth(c).unwrap() != '.')
                || number.next.is_some_and(|c| self.schema.lines().nth(number.line_no).unwrap().chars().nth(c).unwrap() != '.')
                || number.prev_adjacent.as_ref().is_some_and(|s| self.schema.lines().nth(number.line_no-1).unwrap()[s.start..=s.end].chars().any(|c| c != '.'))
                || number.next_adjacent.as_ref().is_some_and(|s| self.schema.lines().nth(number.line_no+1).unwrap()[s.start..=s.end].chars().any(|c| c != '.')) {
                Some(number.value)
            } else {
                None
            }
        }).flatten().collect::<Vec<u64>>()
    }

    // fn solve_2(&self) -> Vec<u64> {
    //     let mut gears: HashMap<u64, Gear> = HashMap::new();
    //     self.numbers.iter().map(|number| {
    //         if let Some(prev) = number.prev {
    //             if self.schema.lines().nth(number.line_no).unwrap().chars().nth(prev).unwrap() == '*' {
    //                 gears.get(number.line_no*self.line_size+prev)
    //             }
    //         }
    //         if number.prev.is_some_and(|c| self.schema.lines().nth(number.line_no).unwrap().chars().nth(c).unwrap() == '*') {
    //             gears.get(number.line_no*self.line_size+c);
    //             gears.into_iter().filter(|gear| gear)
    //         }
    //             || number.next.is_some_and(|c| self.schema.lines().nth(number.line_no).unwrap().chars().nth(c).unwrap() == '*')
    //             || number.prev_adjacent.as_ref().is_some_and(|s| self.schema.lines().nth(number.line_no-1).unwrap()[s.start..=s.end].chars().any(|c| c == '*'))
    //             || number.next_adjacent.as_ref().is_some_and(|s| self.schema.lines().nth(number.line_no+1).unwrap()[s.start..=s.end].chars().any(|c| c == '*')) {
    //             Some(number.value)
    //         } else {
    //             None
    //         }
    //         // Some(number.value)
    //     }).flatten().collect::<Vec<u64>>()
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n\
.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn test_1() {
        let schema = Schematic::new(TEST_INPUT);
        let result = schema.solve_1();
        assert_eq!(vec![467,35,633,617,592,755,664,598], result);
    }

    // #[test]
    // fn test_2() {
    //     // let input = read_input_v2(TEST_INPUT);
    //     let schema = Schematic::new(TEST_INPUT);
    //     let result = schema.solve_2();
    //     assert_eq!(vec![467,35], result);
    // }
}
