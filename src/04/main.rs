use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    println!("--- Day 4: Scratchcards ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let sum =
        input.lines().fold(0, |acc, line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning_numbers_s, my_numbers_s) = numbers.split_once(" | ").unwrap();

            let winning_numbers: HashSet<&str> = winning_numbers_s
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect();
            let my_numbers: HashSet<&str> =
                my_numbers_s.split(" ").filter(|x| !x.is_empty()).collect();

            let score: usize = winning_numbers.intersection(&my_numbers).fold(0, |acc, _| {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            });

            acc + score
        });

    sum
}

fn pt2(input: &str) -> usize {
    let mut copies: HashMap<usize, usize> = HashMap::new();

    input.lines().enumerate().for_each(|(line_i, line)| {
        let card_number = line_i + 1;
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers_s, my_numbers_s) = numbers.split_once(" | ").unwrap();

        let winning_numbers: HashSet<&str> = winning_numbers_s
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();
        let my_numbers: HashSet<&str> = my_numbers_s.split(" ").filter(|x| !x.is_empty()).collect();

        let mut multiplier = 1;
        if let Some(c) = copies.get(&card_number) {
            multiplier = multiplier + c;
        }
        (0..multiplier).for_each(|_| {
            winning_numbers
                .intersection(&my_numbers)
                .enumerate()
                .for_each(|(i, _)| {
                    let next_card_number = card_number + (i + 1);
                    if let Some(win) = copies.get_mut(&next_card_number) {
                        *win += 1;
                    } else {
                        copies.insert(next_card_number, 1);
                    }
                });
        });

        if let Some(card_number) = copies.get_mut(&card_number) {
            *card_number += 1;
        } else {
            copies.insert(card_number, 1);
        };
    });

    let sum = copies
        .iter()
        .fold(0, |acc, (_, copy_count)| acc + copy_count);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 30);
    }
}
