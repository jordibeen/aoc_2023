use std::{io::Write, time::Instant};

fn main() {
    println!("--- Day 13: Point of Incidence ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut horizontal_mirrors: Vec<Vec<Vec<char>>> = Vec::new();
    let mut vertical_mirrors: Vec<Vec<Vec<char>>> = Vec::new();

    let mut input_mirror: Vec<Vec<char>> = Vec::new();
    input.split("\n").for_each(|line| {
        if line.is_empty() {
            horizontal_mirrors.push(input_mirror.to_owned());

            let mut vertical_mirror: Vec<Vec<char>> =
                (0..input_mirror[0].len()).map(|_| vec![]).collect();
            input_mirror.iter().for_each(|row| {
                row.iter()
                    .enumerate()
                    .for_each(|(i, c)| vertical_mirror[i].push(*c));
            });
            vertical_mirrors.push(vertical_mirror);

            input_mirror = Vec::new();
        } else {
            input_mirror.push(line.chars().collect());
        }
    });

    let horizontal_reflections: usize = horizontal_mirrors.iter().fold(0, |acc, mirror| {
        acc + mirror
            .iter()
            .enumerate()
            .map(|(i, row)| {
                if i + 1 <= mirror.len() - 1 && row == &mirror[i + 1] {
                    count_reflections(mirror, i) * 100
                } else {
                    0
                }
            })
            .sum::<usize>()
    });

    let vertical_reflections: usize = vertical_mirrors.iter().fold(0, |acc, mirror| {
        acc + mirror
            .iter()
            .enumerate()
            .map(|(i, col)| {
                if i + 1 <= mirror.len() - 1 && col == &mirror[i + 1] {
                    count_reflections(mirror, i)
                } else {
                    0
                }
            })
            .sum::<usize>()
    });

    horizontal_reflections + vertical_reflections
}

fn count_reflections(mirror: &Vec<Vec<char>>, i: usize) -> usize {
    let reflections: Vec<bool> = mirror[..(i + 1)]
        .iter()
        .rev()
        .zip(mirror[(i + 1)..].iter())
        .map(|(l, r)| l == r)
        .collect();

    if reflections.iter().all(|x| x == &true) {
        return i + 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 405);
    }
}
