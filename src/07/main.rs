use std::{collections::HashMap, time::Instant};

const CHARS: [&'static str; 14] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1",
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum StrengthType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand {
    bid: u16,
    values: (u8, u8, u8, u8, u8),
    strength: StrengthType,
}

impl Hand {
    fn new(hand: (&str, &str), part2: bool) -> Self {
        let v: Vec<u8> = hand
            .0
            .chars()
            .map(|char| {
                if part2 && char == 'J' {
                    0
                } else {
                    CHARS
                        .iter()
                        .rev()
                        .position(|x| x == &char.to_string())
                        .unwrap() as u8
                }
            })
            .collect();

        Self {
            bid: hand.1.parse::<u16>().unwrap(),
            values: (v[0], v[1], v[2], v[3], v[4]),
            strength: Self::calc_strength(hand.0, part2),
        }
    }

    fn calc_strength(cards: &str, part2: bool) -> StrengthType {
        let mut dist: HashMap<char, u16> = HashMap::new();
        for char in cards.chars() {
            dist.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut scores: Vec<u16> = dist
            .keys()
            .map(|key| {
                if part2 && key == &'J' {
                    0_u16
                } else {
                    dist.get(key).unwrap().to_owned()
                }
            })
            .collect();
        scores.sort_by(|a, b| b.cmp(a));

        if part2 {
            scores[0] += cards.chars().filter(|a| a == &'J').count() as u16;
        }

        let strength_type = match scores[0] {
            5 => StrengthType::FiveOfAKind,
            4 => StrengthType::FourOfAKind,
            3 => {
                if scores[1] == 2 {
                    StrengthType::FullHouse
                } else {
                    StrengthType::ThreeOfAKind
                }
            }
            2 => {
                if scores[1] == 2 {
                    StrengthType::TwoPair
                } else {
                    StrengthType::OnePair
                }
            }
            _ => StrengthType::HighCard,
        };

        strength_type
    }
}

fn main() {
    println!("--- Day 7: Camel Cards ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::new(line.split_once(" ").unwrap(), false))
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    winnings
}

fn pt2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::new(line.split_once(" ").unwrap(), true))
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 5905);
    }
}
