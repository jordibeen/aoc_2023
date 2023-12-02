use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    println!("Day 01");
    let input: String = fs::read_to_string("./src/01/input.txt").expect("File should exist");
    pt1(&input);
    pt2(&input);
}

fn pt1(input: &String) -> () {
    println!("Part 1 starting..");
    let start: Instant = Instant::now();

    let sum = input.lines().into_iter().fold(0, |acc, line| {
        let digits: Vec<String> = Regex::new(r"\d")
            .unwrap()
            .find_iter(line)
            .map(|m| m.as_str().to_owned())
            .collect();

        acc + (digits.first().unwrap().to_owned() + digits.last().unwrap())
            .parse::<i32>()
            .unwrap()
    });

    println!("Part 1 finished in {:?}. Answer: {}", start.elapsed(), sum);
}

fn pt2(input: &String) -> () {
    println!("Part 2 starting..");
    let start = Instant::now();

    let sum = input.lines().into_iter().fold(0, |acc, line| {
        let rev_line = line.chars().rev().collect::<String>();
        let first = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
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
            .unwrap();
        let last = Regex::new(r"\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno")
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
            .unwrap();
        acc + (first.to_owned() + last).parse::<i32>().unwrap()
    });

    println!("Part 2 finished in {:?}. Answer: {}", start.elapsed(), sum)
}
