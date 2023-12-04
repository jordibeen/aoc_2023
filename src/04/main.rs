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
    let total = input.lines().fold(0, |total, line| {
        let (winning_numbers_str, my_numbers_str) =
            line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let winning_numbers: HashSet<&str> = winning_numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();
        let my_numbers: HashSet<&str> = my_numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();

        let points: usize = my_numbers
            .intersection(&winning_numbers)
            .fold(0, |points, _| if points == 0 { 1 } else { points * 2 });

        total + points
    });

    total
}

fn pt2(input: &str) -> usize {
    let mut card_counter: HashMap<usize, usize> = HashMap::new();

    input.lines().enumerate().for_each(|(line_i, line)| {
        let card_number = line_i + 1;
        let (winning_numbers_str, my_numbers_str) =
            line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let winning_numbers: HashSet<&str> = winning_numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();
        let my_numbers: HashSet<&str> = my_numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();

        let mut multiplier = 1;
        if let Some(card_count) = card_counter.get(&card_number) {
            multiplier = multiplier + card_count;
        }

        (0..multiplier).for_each(|_| {
            my_numbers
                .intersection(&winning_numbers)
                .enumerate()
                .for_each(|(i, _)| {
                    let next_card_number = card_number + (i + 1);
                    if let Some(copies) = card_counter.get_mut(&next_card_number) {
                        *copies += 1;
                    } else {
                        card_counter.insert(next_card_number, 1);
                    }
                });
        });

        if let Some(copies) = card_counter.get_mut(&card_number) {
            *copies += 1;
        } else {
            card_counter.insert(card_number, 1);
        };
    });

    let total = card_counter.iter().fold(0, |acc, (_, copies)| acc + copies);

    total
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
