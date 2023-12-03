use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

type Position = (i32, i32);

fn main() {
    println!("--- Day 3: Gear Ratios ---");
    let input: String = fs::read_to_string("./src/03/input.txt").expect("File should exist");
    pt1(&input);
    pt2(&input);
}

fn pt1(input: &String) -> () {
    println!("Part 1 starting..");
    let start: Instant = Instant::now();

    let mut part_numbers: Vec<(usize, Match)> = Vec::new();
    let mut symbol_locations: HashMap<Position, &str> = HashMap::new();

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
                let val = find.as_str();
                symbol_locations.insert(pos, val);
            });
    });

    let sum = part_numbers.iter().fold(0, |acc, (row, m)| {
        let mut adjacent_symbols: HashSet<_> = HashSet::new();
        let part_number_val = m.as_str().parse::<usize>().unwrap();

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
                if let Some(adjacent_symbol) = symbol_locations.get(pos) {
                    adjacent_symbols.insert(adjacent_symbol);
                }
            });
        });

        acc + part_number_val * adjacent_symbols.len()
    });

    println!("Part 1 finished in {:?}. Answer: {}", start.elapsed(), sum);
}

fn pt2(input: &String) -> () {
    println!("Part 2 starting..");
    let start: Instant = Instant::now();

    let mut part_numbers: Vec<(usize, Match)> = Vec::new();
    let mut symbol_locations: HashMap<Position, &str> = HashMap::new();

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
                let val = find.as_str();
                symbol_locations.insert(pos, val);
            });
    });

    let mut symbols: HashMap<Position, HashSet<usize>> = HashMap::new();
    part_numbers.iter().for_each(|(row, m)| {
        let part_number_val = m.as_str().parse::<usize>().unwrap();

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
                if let Some(_) = symbol_locations.get(pos) {
                    symbols
                        .entry(*pos)
                        .or_insert(HashSet::new())
                        .insert(part_number_val);
                }
            });
        });
    });

    let sum = symbols.iter().fold(0, |acc, (_, vals)| {
        if vals.len() > 1 {
            let symbol_value: usize = vals
                .into_iter()
                .fold(0, |acc, c| if acc == 0 { *c } else { acc * c });
            acc + symbol_value
        } else {
            acc
        }
    });

    println!("Part 2 finished in {:?}. Answer: {}", start.elapsed(), sum);
}
