use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

type Position = (i32, i32);
type PartValue = usize;

fn main() {
    println!("--- Day 3: Gear Ratios ---");
    let input: String = fs::read_to_string("./src/03/input.txt").expect("File should exist");
    let start: Instant = Instant::now();

    let mut part_numbers: Vec<(PartValue, Match)> = Vec::new();
    let mut gears: HashMap<Position, HashSet<PartValue>> = HashMap::new();

    input.lines().enumerate().for_each(|(row_i, line)| {
        Regex::new(r"\d+")
            .unwrap()
            .captures_iter(line)
            .for_each(|cap| {
                let part_number = cap.get(0).unwrap();
                part_numbers.push((row_i, part_number));
            });

        Regex::new(r"[^\d|\.]")
            .unwrap()
            .find_iter(line)
            .for_each(|find| {
                let gear_pos = (find.start() as i32, row_i as i32);
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

    let pt1 = gears.iter().fold(0, |acc, (_, part_values)| {
        acc + part_values.iter().sum::<PartValue>()
    });

    let pt2 = gears.iter().fold(0, |acc, (_, part_values)| {
        if part_values.len() > 1 {
            let gear_value = part_values
                .iter()
                .fold(0, |acc, c| if acc == 0 { *c } else { acc * c });
            acc + gear_value
        } else {
            acc
        }
    });

    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
    println!("Ran for {:?}", start.elapsed());
}
