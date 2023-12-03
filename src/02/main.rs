use regex::Regex;
use std::time::Instant;

fn main() {
    println!("--- Day 2: Cube Conundrum ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
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

    sum
}

fn pt2(input: &str) -> i32 {
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

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 2286);
    }
}
