use core::num;
use std::borrow::Borrow;

use regex::{Match, Regex};

pub fn day03_1() {
    let input_v1 = read_input_v1(include_str!("../inputs/day03.txt"));
    println!("day 3a: {}", input_v1.iter().sum::<u64>());

    // let input_v2 = read_input_v2(include_str!("../../inputs/day02.txt"));
    // println!("day 2b: {}", input_v2.iter().sum::<u64>());
}

// enum PartType {
//     Symbol,
//     Gear,
//     Number(u64),
// }
//
// struct Part {
//     line_no: usize,
//     offset: usize,
//     value: PartType,
// }
//
// impl From<Match> for Part {
//     fn from(value: Match) -> Self {
//         let s = value.as_str();
//     }
// }
//
// struct Engine<'a> {
//     gear_pat: Regex,
//     symbol_pat: Regex,
//     number_pat: Regex,
//     symbols: Vec<Vec<Part>>,
//     gears: Vec<Vec<Part>>,
//     numbers: Vec<Vec<Part>>,
//     schematics: &'a str,
// }
//
// impl Engine {
//     fn new(schematic: &str) -> Self {
//         let lines_count = schematic.lines().count();
//         Self {
//             gear_pat: Regex::new(r"\*+").unwrap(),
//             symbol_pat: Regex::new(r"\w+").unwrap(),
//             number_pat: Regex::new(r"\d+").unwrap(),
//             symbols: Vec::with_capacity(lines_count),
//             gears: Vec::with_capacity(lines_count),
//             numbers: Vec::with_capacity(lines_count),
//             schematics: schematic
//         }
//     }
//
//     fn run_v1(&self) -> Vec<u64> {
//         let lines = schematic.lines();
//         let lines_count = lines.count();
//         let mut symbols: Vec<Vec<(usize, char)>> = vec![];
//         let mut numbers: Vec<Vec<(usize, u64)>> = vec![];
//         for (line_idx, line) in lines.int {
//             let line_symbols: Vec<(usize, char)> = self._find_matches(line, self.gear_pat);
//             let line_numbers: Vec<(usize, u64)> = self._find_matches(line, self.number_pat);
//             symbols.push(line_symbols);
//             numbers.push(line_numbers);
//         }
//     }
//
//     fn _initialize(&self) {
//         for line in self.schematics.lines() {
//             let line_symbols = self._find_matches(line, &self.symbol_pat);
//             let line_gears = self._find_matches(line, &self.gear_pat);
//             let line_numbers = self._find_matches(line, &self.number_pat);
//             self.symbols.push(line_symbols);
//             self.gears.push(line_gears);
//             self.numbers.push(line_numbers);
//         }
//     }
//
//     fn _find_matches(&self, line: &str, re: &Regex) -> Vec<Part>
//     {
//         re.find_iter(line)
//             .map(|m| {
//                 m.into()
//             })
//             .collect()
//     }
// }

struct Parser {
    number_pat: Regex
}

impl Parser {
    fn new() -> Self {
        Self {
            number_pat: Regex::new(r"(\d+)").unwrap(),
        }
    }

    fn parse(&self, input: &str) -> Vec<u64> {
        let line_size = input.find("\n").unwrap();
        let new_input = input.replace("\n", "");
        self.number_pat
        .find_iter(new_input.as_str())
        .map(|m| {
            let line_no = m.start()/line_size;
            let start_offset = m.start()%line_size;
            let end_offset = (m.end()-1)%line_size;
            let number = m.as_str().parse::<u64>().unwrap();

            let previous = if start_offset > 0 { new_input.chars().nth(line_no*line_size+start_offset-1) } else { None };
            let previous_up_diag = if start_offset > 0 && line_no > 0 { new_input.chars().nth((line_no-1)*line_size+start_offset-1) } else { None };
            let previous_down_diag = if start_offset > 0 && (line_no+1)*line_size < new_input.len() { new_input.chars().nth((line_no+1)*line_size+start_offset-1) } else { None };

            let next = if end_offset+1 < line_size { new_input.chars().nth(line_no*line_size+end_offset+1) } else { None };
            let next_up_diag = if end_offset < line_size && line_no > 0 { new_input.chars().nth((line_no-1)*line_size+end_offset+1) } else { None };
            let next_down_diag = if end_offset < line_size && (line_no+1)*line_size < new_input.len() { new_input.chars().nth((line_no+1)*line_size+end_offset+1) } else { None };
            if previous.is_some_and(|c| c != '.')
            || previous_up_diag.is_some_and(|c| c != '.')
            || previous_down_diag.is_some_and(|c| c != '.')
            || next.is_some_and(|c| c != '.')
            || next_up_diag.is_some_and(|c| c != '.')
            || next_down_diag.is_some_and(|c| c != '.') {
                Some(number)
            } else {
                let mut ok = false;
                for i in start_offset..end_offset+1 {
                    if line_no > 0 && new_input.chars().nth((line_no-1)*line_size+i).unwrap() != '.' {
                        ok = true;
                    }
                    if (line_no+1)*line_size < new_input.len() && new_input.chars().nth((line_no+1)*line_size+i).unwrap() != '.' {
                        ok = true;
                    }
                }
                if ok {
                    Some(number)
                } else {
                    None
                }
            }

        })
        .flatten()
        .collect::<Vec<u64>>()
    }
}

fn read_input_v1(input: &str) -> Vec<u64> {
    let parser = Parser::new();
    parser.parse(input)
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

    // #[test]
    // fn test_1bis() {
    //     let engine = Engine::new(TEST_INPUT);
    //     let output = engine.run_v1();
    //     assert_eq!(vec![467,35,633,617,592,755,664,598], output);
    // }
}
