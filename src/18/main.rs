use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    time::Instant,
};

type Coords = (i32, i32);

fn main() {
    println!("--- Day 18: Lavaduct Lagoon ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let regex = Regex::new("(?<direction>[A-Z]) (?<meters>\\d+) \\((?<rgb>#[a-z0-9]+)\\)").unwrap();
    let mut pos: Coords = (0, 0);
    let mut cubes: HashMap<Coords, String> = HashMap::new();

    // (min_x, min_y, max_x, max_y)
    let mut dimensions: (i32, i32, i32, i32) = (0, 0, 0, 0);

    input.lines().for_each(|line| {
        let caps = regex.captures(line).unwrap();

        let direction = match caps.name("direction").unwrap().as_str() {
            "U" => (-1, 0),
            "R" => (0, 1),
            "D" => (1, 0),
            "L" => (0, -1),
            _ => unreachable!(),
        };
        let meters = caps.name("meters").unwrap().as_str().parse::<i8>().unwrap();
        let rgb = caps.name("rgb").unwrap().as_str();

        (0..meters).for_each(|_| {
            let next_pos = ((pos.0 + direction.0), (pos.1 + direction.1));
            dimensions = (
                min(dimensions.0, next_pos.1),
                min(dimensions.1, next_pos.0),
                max(dimensions.2, next_pos.1 + 1),
                max(dimensions.3, next_pos.0 + 1),
            );

            cubes.insert(next_pos, rgb.to_string());
            pos = next_pos;
        });
    });

    flood(&mut cubes);

    // Debug grid
    // (dimensions.1..dimensions.3).for_each(|y| {
    //     println!("");
    //     (dimensions.0..dimensions.2).for_each(|x| {
    //         if cubes.get(&(y, x)).is_some() {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     });
    // });

    cubes.len()
}

fn flood(cubes: &mut HashMap<Coords, String>) -> () {
    let mut q: VecDeque<(Coords, &str)> = VecDeque::new();
    q.push_back(((1, 1), "#000000"));

    while let Some((coords, fill)) = q.pop_front() {
        if cubes.get(&coords).is_some() {
            continue;
        };
        cubes.insert(coords, fill.to_string());
        q.push_back(((coords.0 + 1, coords.1), fill));
        q.push_back(((coords.0 - 1, coords.1), fill));
        q.push_back(((coords.0, coords.1 + 1), fill));
        q.push_back(((coords.0, coords.1 - 1), fill));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 62);
    }
}
