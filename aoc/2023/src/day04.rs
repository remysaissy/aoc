use std::str::FromStr;
use std::usize;

pub fn day04_a(input: &str) -> u64 {
    let cardboard = CardBoard::from_str(input).unwrap();
    let result = cardboard.solve_1();
    result.iter().sum()
}

pub fn day04_b(input: &str) -> u64 {
    let cardboard = CardBoard::from_str(input).unwrap();
    let result = cardboard.solve_2();
    result.iter().sum()
}

#[derive(Debug, PartialEq)]
struct Card {
    card_no: usize,
    winning_numbers: Vec<u64>,
    owned_numbers: Vec<u64>,
}

impl Card {
    fn find_matches(&self) -> Vec<u64> {
        let mut matches: Vec<u64> = Vec::new();
        let mut start = 0;
        let end = self.owned_numbers.len();
        for (idx, winning_number) in self.winning_numbers.iter().enumerate() {
            while start < end {
                let owned_number = self.owned_numbers[start];
                if owned_number == *winning_number {
                    matches.push(owned_number);
                } else if owned_number > *winning_number {
                    break;
                }
                start += 1;
            }
        }
        matches
    }

    fn calculate_points(&self, matches: &Vec<u64>) -> u64 {
        matches.iter().fold(0, |acc, _| {
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        })
    }

    fn parse_numbers(line: &str) -> Result<Vec<u64>, CardError> {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut numbers: Vec<u64> = Vec::new();
        for (idx, c) in line.chars().enumerate() {
            let is_digit = c.is_digit(10);
            if is_digit {
                if start.is_none() {
                    start = Some(idx);
                }
                end = Some(idx);
            } else {
                if start.is_some() {
                    let slice = &line[start.unwrap()..=end.unwrap()];
                    let number: u64 = slice.parse().map_err(|_| CardError)?;
                    numbers.push(number);
                    start = None;
                    end = None;
                }
            }
        }
        // Final number
        if start.is_some() {
            let slice = &line[start.unwrap()..=end.unwrap()];
            let number: u64 = slice.parse().map_err(|_| CardError)?;
            numbers.push(number);
        }
        numbers.sort();
        Ok(numbers)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CardError;

impl FromStr for Card {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, numbers) = s.split_once(':').unwrap();
        let (_, card_no) = header.split_once(' ').unwrap();
        let (winning, owned) = numbers.split_once('|').unwrap();
        Ok(Card {
            card_no: card_no.trim().parse().map_err(|_| CardError).unwrap(),
            winning_numbers: Self::parse_numbers(winning).map_err(|_| CardError).unwrap(),
            owned_numbers: Self::parse_numbers(owned).map_err(|_| CardError).unwrap(),
        })
    }
}

#[derive(Debug, PartialEq)]
struct CardBoard {
    cards: Vec<Card>
}

impl CardBoard {
    fn solve_1(&self) -> Vec<u64> {
        self.cards.iter().map(|card| {
            let matches = card.find_matches();
            card.calculate_points(&matches)
        }).collect()
    }

    fn solve_2(&self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::with_capacity(self.cards.len());
        for _ in 0..self.cards.len() {
            result.push(1);
        }
        for card in self.cards.iter() {
            let matches = card.find_matches();
            if matches.len() > 0 {
                let start = card.card_no;
                let mut end = start + matches.len();
                if end > self.cards.len() {
                    end = self.cards.len();
                }
                let number_of_copies = result[start-1];
                for idx in start..end {
                    result[idx as usize] += number_of_copies;
                }
            }
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CardBoardError;

impl FromStr for CardBoard {
    type Err = CardBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.lines().map(|line| Card::from_str(line).map_err(|_| CardBoardError).unwrap()).collect();
        Ok(CardBoard {
            cards
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_init() {
        let cardboard = CardBoard::from_str(TEST_INPUT).unwrap();
        assert_eq!(6, cardboard.cards.len());

        assert_eq!(vec![17, 41, 48, 83, 86], cardboard.cards[0].winning_numbers);
        assert_eq!(vec![6, 9, 17, 31, 48, 53, 83, 86], cardboard.cards[0].owned_numbers);
    }

    #[test]
    fn test_1() {
        let cardboard = CardBoard::from_str(TEST_INPUT).unwrap();
        let result = cardboard.solve_1();
        assert_eq!(vec![8,2,2,1,0,0], result);
    }

    #[test]
    fn test_2() {
        let cardboard = CardBoard::from_str(TEST_INPUT).unwrap();
        let result = cardboard.solve_2();
        assert_eq!(vec![1,2,4,8,14,1], result);
    }
}
