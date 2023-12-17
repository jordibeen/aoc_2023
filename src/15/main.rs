use std::{collections::HashMap, time::Instant};

fn main() {
    println!("--- Day 15: Lens Library ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    input.split(",").map(|step| hash(step.trim())).sum()
}

fn pt2(input: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<&str>> = HashMap::new();

    input.split(",").for_each(|step| {
        let step: &str = step.trim();

        if let Some((label, _)) = step.split_once("=") {
            let box_number: usize = hash(label);
            if let Some(b) = boxes.get_mut(&box_number) {
                if let Some(i) = b.iter().position(|x| x.contains(label)) {
                    b[i] = step;
                } else {
                    b.push(step);
                }
            } else {
                boxes.insert(box_number, vec![step]);
            }
        }

        if let Some((label, _)) = step.split_once("-") {
            let box_number: usize = hash(label);
            if let Some(b) = boxes.get_mut(&box_number) {
                b.retain(|x| !x.contains(label));
            }
        }
    });

    let sum = boxes.iter().fold(0, |acc, (b, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| {
                let (_, focal_length) = lens.split_once("=").unwrap();
                let lens_power = (b + 1) * (i + 1) * focal_length.parse::<usize>().unwrap();

                lens_power
            })
            .sum::<usize>()
    });

    sum
}

fn hash(string: &str) -> usize {
    string
        .as_bytes()
        .iter()
        .fold(0, |acc, ascii_code| (acc + *ascii_code as usize) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn pt1_test_2() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 1320);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 145);
    }
}
