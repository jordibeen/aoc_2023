use std::time::Instant;

fn main() {
    println!("--- Day 9: Mirage Maintenance ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 1: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> isize {
    let mut histories: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    // let mut differences: Vec<Vec<isize>> = Vec::new();
    let sequences: Vec<Vec<Vec<isize>>> = histories
        .iter()
        .map(|history| {
            let mut diff_sequences: Vec<Vec<isize>> = Vec::new();

            let mut diff_sequence: Vec<isize> = history.clone();
            while !diff_sequence.iter().all(|x| x == &0_isize) {
                diff_sequence = diff_sequence
                    .iter()
                    .enumerate()
                    .filter_map(|(i, value)| {
                        if i < diff_sequence.len() - 1 {
                            let next_value = diff_sequence[i + 1];
                            let diff = next_value - value;
                            Some(diff)
                        } else {
                            None
                        }
                    })
                    .collect();
                diff_sequences.push(diff_sequence.clone());
            }

            diff_sequences
        })
        .collect();

    let extrapolations: Vec<isize> = histories
        .iter_mut()
        .zip(&sequences)
        .map(|(history, diff_sequences)| {
            let extrapolation_val = diff_sequences
                .iter()
                .rev()
                .fold(0, |acc, sequence| acc + sequence.last().unwrap());

            history.last().unwrap() + extrapolation_val
        })
        .collect();

    extrapolations.iter().sum()
}

fn pt2(input: &str) -> isize {
    let mut histories: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    // let mut differences: Vec<Vec<isize>> = Vec::new();
    let sequences: Vec<Vec<Vec<isize>>> = histories
        .iter()
        .map(|history| {
            let mut diff_sequences: Vec<Vec<isize>> = Vec::new();

            let mut diff_sequence: Vec<isize> = history.clone();
            while !diff_sequence.iter().all(|x| x == &0_isize) {
                diff_sequences.push(diff_sequence.clone());
                diff_sequence = diff_sequence
                    .iter()
                    .enumerate()
                    .filter_map(|(i, value)| {
                        if i < diff_sequence.len() - 1 {
                            let next_value = diff_sequence[i + 1];
                            let diff = next_value - value;
                            Some(diff)
                        } else {
                            None
                        }
                    })
                    .collect();
            }

            diff_sequences
        })
        .collect();

    let extrapolations: Vec<isize> = histories
        .iter_mut()
        .zip(&sequences)
        .map(|(_, diff_sequences)| {
            let extrapolation_val = diff_sequences
                .iter()
                .rev()
                .fold(0, |acc, seq| seq.first().unwrap() - acc);

            extrapolation_val
        })
        .collect();

    extrapolations.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 114);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 2);
    }
}
