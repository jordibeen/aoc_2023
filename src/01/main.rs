use regex::Regex;
use std::time::Instant;

fn main() {
    println!("--- Day 1: Trebuchet?! ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let sum: i32 = input.lines().into_iter().fold(0, |acc, line| {
        let digits: Vec<String> = Regex::new(r"\d")
            .unwrap()
            .find_iter(line)
            .map(|m| m.as_str().to_owned())
            .collect();

        acc + (digits.first().unwrap().to_owned() + digits.last().unwrap())
            .parse::<i32>()
            .unwrap()
    });

    sum
}

fn pt2(input: &str) -> i32 {
    let sum: i32 = input.lines().into_iter().fold(0, |acc, line| {
        let rev_line = line.chars().rev().collect::<String>();
        let first: String = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
            .unwrap()
            .find(line)
            .map(|x| match x.as_str() {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => x.as_str(),
            })
            .unwrap()
            .to_owned();
        let last: String = Regex::new(r"\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno")
            .unwrap()
            .find(&rev_line)
            .map(|x| match x.as_str() {
                "eno" => "1",
                "owt" => "2",
                "eerht" => "3",
                "ruof" => "4",
                "evif" => "5",
                "xis" => "6",
                "neves" => "7",
                "thgie" => "8",
                "enin" => "9",
                _ => x.as_str(),
            })
            .unwrap()
            .to_owned();
        acc + (first + &last).parse::<i32>().unwrap()
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 142);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example_pt2.txt");
        let result = pt2(&input);
        assert_eq!(result, 281);
    }
}
