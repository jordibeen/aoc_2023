use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Position = (i32, i32);
type PartValue = usize;
type Gears = HashMap<Position, HashSet<PartValue>>;

fn main() {
    println!("--- Day 3: Gear Ratios ---");
    let start: Instant = Instant::now();

    let input = include_str!("./input.txt");
    let gears = parse(&input);
    println!("Part 1: {}", pt1(&gears));
    println!("Part 2: {}", pt2(&gears));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn parse(input: &str) -> Gears {
    let mut gears: Gears = HashMap::new();
    let mut part_numbers: Vec<(PartValue, Match)> = Vec::new();

    input.lines().enumerate().for_each(|(row_i, line)| {
        Regex::new(r"\d+")
            .unwrap()
            .captures_iter(line)
            .for_each(|cap| {
                let part_number: Match = cap.get(0).unwrap();
                part_numbers.push((row_i, part_number));
            });

        Regex::new(r"[^\d|\.]")
            .unwrap()
            .find_iter(line)
            .for_each(|find| {
                let gear_pos: Position = (find.start() as i32, row_i as i32);
                gears.insert(gear_pos, HashSet::new());
            });
    });

    part_numbers.iter().for_each(|(row, m)| {
        (m.start()..m.end()).for_each(|x| {
            let pos = (x as i32, *row as i32);
            vec![
                (pos.0 - 1, pos.1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
                (pos.0 + 1, pos.1),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 1, pos.1 - 1),
            ]
            .iter()
            .for_each(|pos| {
                if let Some(gear) = gears.get_mut(pos) {
                    let part_value = m.as_str().parse::<PartValue>().unwrap();
                    gear.insert(part_value);
                }
            });
        });
    });

    gears
}

fn pt1(gears: &Gears) -> usize {
    let sum: usize = gears.iter().fold(0, |acc, (_, part_values)| {
        acc + part_values.iter().sum::<PartValue>()
    });

    sum
}

fn pt2(gears: &Gears) -> usize {
    let sum = gears.iter().fold(0, |gear_ratios, (_, part_values)| {
        if part_values.len() > 1 {
            let gear_value = part_values.iter().fold(0, |gear_ratios, part_value| {
                if gear_ratios == 0 {
                    *part_value
                } else {
                    gear_ratios * part_value
                }
            });
            gear_ratios + gear_value
        } else {
            gear_ratios
        }
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let gears = parse(&input);
        let result = pt1(&gears);
        assert_eq!(result, 4361);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let gears = parse(&input);
        let result = pt2(&gears);
        assert_eq!(result, 467835);
    }
}
