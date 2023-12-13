use std::{collections::HashMap, time::Instant};

fn main() {
    println!("--- Day 12: Hot Springs ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn check_characters(
    characters: &Vec<char>,
    groups: &Vec<usize>,
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if characters.len() == 0 {
        if groups.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    match characters[0] {
        '.' => check_characters(&characters[1..].to_owned(), groups, cache),
        '#' => check_hash_sequence(groups, characters, cache),
        '?' => {
            check_characters(&characters[1..].to_owned(), groups, cache)
                + check_hash_sequence(groups, characters, cache)
        }
        _ => panic!("unhandled"),
    }
}

fn check_hash_sequence(
    groups: &Vec<usize>,
    characters: &Vec<char>,
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if let Some(&result) = cache.get(&(characters.to_owned(), groups.to_owned())) {
        return result;
    }

    if groups.len() == 0 {
        return 0;
    }

    let max = groups[0];
    if characters.len() < max {
        return 0;
    }

    for i in 0..max {
        if characters[i] == '.' {
            return 0;
        }
    }

    if characters.len() == max {
        if groups.len() == 1 {
            return 1;
        }
        return 0;
    }

    if characters[max] == '#' {
        return 0;
    }

    let result = check_characters(
        &characters[(max + 1)..].to_owned(),
        &groups[1..].to_owned(),
        cache,
    );
    cache.insert((characters.to_owned(), groups.to_owned()), result);
    result
}

fn pt1(input: &str) -> usize {
    let mut cache = HashMap::new();

    let sum: usize = input.lines().fold(0, |acc, line| {
        let (condition, separations) = line.split_once(" ").unwrap();
        let groups: Vec<usize> = separations
            .split(",")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        acc + check_characters(&condition.chars().collect(), &groups, &mut cache)
    });

    sum
}

fn pt2(input: &str) -> usize {
    let mut cache = HashMap::new();

    let sum: usize = input.lines().fold(0, |acc, line| {
        let (condition, separations) = line.split_once(" ").unwrap();
        let groups: Vec<usize> = separations
            .split(",")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        let unfolded_condition = (0..5)
            .map(|_| condition.to_string())
            .collect::<Vec<String>>()
            .join("?");
        let unfolded_groups: Vec<usize> = (0..5).map(|_| groups.to_owned()).flatten().collect();

        acc + check_characters(
            &unfolded_condition.chars().collect(),
            &unfolded_groups,
            &mut cache,
        )
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
        assert_eq!(result, 21);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 525152);
    }
}
