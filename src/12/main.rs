use std::time::Instant;

fn main() {
    println!("--- Day 12: Hot Springs ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn check_characters(characters: &Vec<char>, groups: &Vec<usize>) -> usize {
    if characters.len() == 0 {
        if groups.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    match characters[0] {
        '.' => check_characters(&characters[1..].to_owned(), groups),
        '#' => check_hash_sequence(characters, groups),
        '?' => {
            check_characters(&characters[1..].to_owned(), groups)
                + check_hash_sequence(characters, groups)
        }
        _ => panic!("unhandled"),
    }
}

fn check_hash_sequence(characters: &Vec<char>, groups: &Vec<usize>) -> usize {
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

    check_characters(&characters[(max + 1)..].to_owned(), &groups[1..].to_owned())
}

fn pt1(input: &str) -> usize {
    let sum: usize = input.lines().fold(0, |acc, line| {
        let (condition, separations) = line.split_once(" ").unwrap();
        let groups: Vec<usize> = separations
            .split(",")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        acc + check_characters(&condition.chars().collect(), &groups)
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
}
