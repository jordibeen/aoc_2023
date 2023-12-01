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

    let mut calibration_value: i32 = 0;
    input.lines().for_each(|line: &str| {
        let digits: Vec<String> = Regex::new(r"\d")
            .unwrap()
            .find_iter(line)
            .map(|m| m.as_str().to_owned())
            .collect();
        let combined_str = digits.first().unwrap().to_owned() + digits.last().unwrap();
        calibration_value += combined_str.parse::<i32>().unwrap();
    });

    println!(
        "Part 1 finished in {:?}. Answer: {}",
        start.elapsed(),
        calibration_value
    );
}

fn pt2(input: &String) -> () {
    println!("Part 2 starting..");
    let start = Instant::now();
    let replacements = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut calibration_value: i32 = 0;
    input.lines().for_each(|line| {
        let mut replacement_line = line.to_owned();
        for (s, r) in replacements {
            replacement_line = replacement_line.replace(s, r);
        }
        let digits: Vec<String> = Regex::new(r"\d")
            .unwrap()
            .find_iter(&replacement_line)
            .map(|m| m.as_str().to_owned())
            .collect();
        let combined_str = digits.first().unwrap().to_owned() + digits.last().unwrap();
        calibration_value += combined_str.parse::<i32>().unwrap();
    });
    println!(
        "Part 2 finished in {:?}. Answer: {}",
        start.elapsed(),
        calibration_value
    )
}
