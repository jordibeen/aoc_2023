use std::collections::HashSet;
use std::time::Instant;

fn main() {
    println!("--- Day 4: Scratchcards ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 13);
    }
}
