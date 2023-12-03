use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    println!("--- Day 2: Cube Conundrum ---");
    let input: String = fs::read_to_string("./src/02/input.txt").expect("File should exist");
    pt1(&input);
    pt2(&input);
}

fn pt1(input: &String) -> () {
    println!("Part 1 starting..");
    let start: Instant = Instant::now();

    let sum: i32 = input.lines().into_iter().fold(0, |acc, line| {
        let (game, cubes): (&str, &str) = line.split_once(": ").unwrap();

        let game_id: i32 = Regex::new(r"Game (?P<id>\d+)")
            .unwrap()
            .captures(game)
            .unwrap()
            .name("id")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let reds: Vec<i32> = Regex::new(r"(?<red>\d+) red")
            .unwrap()
            .find_iter(cubes)
            .filter_map(|c| c.as_str().replace(" red", "").parse::<i32>().ok())
            .collect();
        let greens: Vec<i32> = Regex::new(r"(?<green>\d+) green")
            .unwrap()
            .find_iter(cubes)
            .filter_map(|c| c.as_str().replace(" green", "").parse::<i32>().ok())
            .collect();
        let blues: Vec<i32> = Regex::new(r"(?<blue>\d+) blue")
            .unwrap()
            .find_iter(cubes)
            .filter_map(|c| c.as_str().replace(" blue", "").parse::<i32>().ok())
            .collect();

        if reds.iter().max().unwrap() <= &12
            && greens.iter().max().unwrap() <= &13
            && blues.iter().max().unwrap() <= &14
        {
            acc + game_id
        } else {
            acc
        }
    });

    println!("Part 1 finished in {:?}. Answer: {}", start.elapsed(), sum);
}

fn pt2(input: &String) -> () {
    let start: Instant = Instant::now();
    println!("Part 2 starting..");

    let sum: i32 = input.lines().into_iter().fold(0, |acc, line| {
        let reds: Vec<i32> = Regex::new(r"(?<red>\d+) red")
            .unwrap()
            .find_iter(line)
            .filter_map(|c| c.as_str().replace(" red", "").parse::<i32>().ok())
            .collect();
        let greens: Vec<i32> = Regex::new(r"(?<green>\d+) green")
            .unwrap()
            .find_iter(line)
            .filter_map(|c| c.as_str().replace(" green", "").parse::<i32>().ok())
            .collect();
        let blues: Vec<i32> = Regex::new(r"(?<blue>\d+) blue")
            .unwrap()
            .find_iter(line)
            .filter_map(|c| c.as_str().replace(" blue", "").parse::<i32>().ok())
            .collect();

        acc + reds.iter().max().unwrap()
            * greens.iter().max().unwrap()
            * blues.iter().max().unwrap()
    });

    println!("Part 2 finished in {:?}. Answer: {}", start.elapsed(), sum);
}