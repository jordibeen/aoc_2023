use std::{collections::HashMap, time::Instant};

const CHARS: [&'static str; 14] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1",
];

#[derive(Copy, Clone, Debug)]
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
    cards: String,
    bid: u16,
    strength: StrengthType,
}

impl Hand {
    fn new(hand: (&str, &str), part2: bool) -> Self {
        Self {
            cards: hand.0.to_owned(),
            bid: hand.1.parse::<u16>().unwrap(),
            strength: Self::calc_strength(hand.0, part2),
        }
    }

    fn calc_strength(cards: &str, part2: bool) -> StrengthType {
        let mut dist: HashMap<char, u16> = HashMap::new();
        cards.chars().for_each(|char| {
            let _ = dist.entry(char).and_modify(|x| *x += 1).or_insert(1);
        });

        let mut scoring: Vec<(u16, &char)> = Vec::new();
        if !part2 {
            dist.keys().for_each(|key| {
                let val = dist.get(key).unwrap();
                match val {
                    &5 => scoring.push((*val, key)),
                    &4 => scoring.push((*val, key)),
                    &3 => scoring.push((*val, key)),
                    &2 => scoring.push((*val, key)),
                    &1 => scoring.push((*val, key)),
                    _ => (),
                }
            });
            scoring.sort_by(|a, b| b.0.cmp(&a.0));
        } else {
            dist.keys().for_each(|key| {
                if key != &'J' {
                    let val = dist.get(key).unwrap();
                    match val {
                        &5 => scoring.push((*val, key)),
                        &4 => scoring.push((*val, key)),
                        &3 => scoring.push((*val, key)),
                        &2 => scoring.push((*val, key)),
                        &1 => scoring.push((*val, key)),
                        _ => (),
                    }
                } else {
                    scoring.push((0, &'A'));
                }
            });
            scoring.sort_by(|a, b| b.0.cmp(&a.0));

            let jokers: u16 = cards.chars().filter(|a| a == &'J').count() as u16;
            if jokers > 0 {
                scoring[0].0 += jokers;
            }
        }

        let highest_count = scoring[0].0;
        match highest_count {
            5 => return StrengthType::FiveOfAKind,
            4 => return StrengthType::FourOfAKind,
            3 => {
                for score in scoring {
                    if score.0 == 2 {
                        return StrengthType::FullHouse;
                    }
                }
                return StrengthType::ThreeOfAKind;
            }
            2 => {
                for score in scoring[1..].iter() {
                    if score.0 == 2 {
                        return StrengthType::TwoPair;
                    }
                }
                return StrengthType::OnePair;
            }
            _ => return StrengthType::HighCard,
        }
    }
}

fn sort_hands(hands: &mut Vec<Hand>, part2: bool) -> &mut Vec<Hand> {
    hands.sort_by(|a, b| {
        let a_strength = a.strength as u8;
        let b_strength = b.strength as u8;

        if a_strength == b_strength {
            let mut a_val: u8 = 0;
            let mut b_val: u8 = 0;
            for (a_char, b_char) in a.cards.chars().zip(b.cards.chars()) {
                let mut a_score = CHARS
                    .iter()
                    .rev()
                    .position(|x| x == &a_char.to_string())
                    .unwrap();
                let mut b_score = CHARS
                    .iter()
                    .rev()
                    .position(|x| x == &b_char.to_string())
                    .unwrap();
                if part2 {
                    if a_char == 'J' {
                        a_score = 0;
                    }
                    if b_char == 'J' {
                        b_score = 0
                    }
                }
                if a_score != b_score {
                    a_val = a_score as u8;
                    b_val = b_score as u8;
                    break;
                }
            }
            a_val.cmp(&b_val)
        } else {
            a_strength.cmp(&b_strength)
        }
    });

    hands
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

    let sorted_hands = sort_hands(&mut hands, false);

    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)))
}

fn pt2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::new(line.split_once(" ").unwrap(), true))
        .collect();

    let sorted_hands = sort_hands(&mut hands, true);

    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)))
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