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
    let mut symbols: HashMap<Position, HashSet<PartValue>> = HashMap::new();

    input.lines().enumerate().for_each(|(row_i, line)| {
        Regex::new(r"\d+")
            .unwrap()
            .captures_iter(line)
            .for_each(|cap| {
                if let Some(part_match) = cap.get(0) {
                    part_numbers.push((row_i, part_match));
                };
            });

        Regex::new(r"[^\d|\.]")
            .unwrap()
            .find_iter(line)
            .for_each(|find| {
                let pos = (find.start() as i32, row_i as i32);
                symbols.insert(pos, HashSet::new());
            });
    });

    part_numbers.iter().for_each(|(row, m)| {
        (m.start()..m.end()).for_each(|x| {
            let part_number_pos = (x as i32, row.to_owned() as i32);
            vec![
                (part_number_pos.0 - 1, part_number_pos.1),
                (part_number_pos.0 - 1, part_number_pos.1 + 1),
                (part_number_pos.0 - 1, part_number_pos.1 - 1),
                (part_number_pos.0, part_number_pos.1 + 1),
                (part_number_pos.0, part_number_pos.1 - 1),
                (part_number_pos.0 + 1, part_number_pos.1),
                (part_number_pos.0 + 1, part_number_pos.1 + 1),
                (part_number_pos.0 + 1, part_number_pos.1 - 1),
            ]
            .iter()
            .for_each(|pos| {
                if let Some(symbol) = symbols.get_mut(pos) {
                    symbol.insert(m.as_str().parse::<PartValue>().unwrap());
                }
            });
        });
    });

    let pt1 = symbols
        .iter()
        .fold(0, |acc, (_, values)| acc + values.iter().sum::<PartValue>());

    let pt2 = symbols.iter().fold(0, |acc, (_, vals)| {
        if vals.len() > 1 {
            let symbol_value = vals
                .iter()
                .fold(0, |acc, c| if acc == 0 { *c } else { acc * c });
            acc + symbol_value
        } else {
            acc
        }
    });

    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
    println!("Ran for {:?}", start.elapsed());
}
