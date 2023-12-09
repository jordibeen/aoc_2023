use std::time::Instant;

type Sequences = Vec<Vec<Vec<isize>>>;

fn main() {
    println!("--- Day 9: Mirage Maintenance ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    let sequences: Sequences = parse(&input);
    println!("Part 1: {}", pt1(&sequences));
    println!("Part 2: {}", pt2(&sequences));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn parse(input: &str) -> Sequences {
    let sequences: Sequences = input
        .lines()
        .map(|line| {
            let history: Vec<isize> = line
                .split(" ")
                .map(|value| value.parse::<isize>().unwrap())
                .collect();

            let mut diff_seqs: Vec<Vec<isize>> = vec![history.clone()];

            let mut curr_seq: Vec<isize> = history.clone();
            while !curr_seq.iter().all(|x| x == &0_isize) {
                curr_seq = curr_seq
                    .iter()
                    .enumerate()
                    .filter_map(|(i, value)| {
                        if i < curr_seq.len() - 1 {
                            let next_value = curr_seq[i + 1];
                            let diff = next_value - value;
                            Some(diff)
                        } else {
                            None
                        }
                    })
                    .collect();
                diff_seqs.push(curr_seq.clone());
            }

            diff_seqs
        })
        .collect();

    sequences
}

fn pt1(sequences: &Sequences) -> isize {
    let answer = sequences.iter().fold(0, |acc, diff_seqs| {
        let extrapolation_value = diff_seqs
            .iter()
            .rev()
            .fold(0, |ext, seq| ext + seq.last().unwrap());

        acc + extrapolation_value
    });

    answer
}

fn pt2(sequences: &Sequences) -> isize {
    let answer = sequences.iter().fold(0, |acc, diff_seqs| {
        let extrapolation_value = diff_seqs
            .iter()
            .rev()
            .fold(0, |ext, seq| seq.first().unwrap() - ext);

        acc + extrapolation_value
    });

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let sequences: Sequences = parse(&input);
        let result = pt1(&sequences);
        assert_eq!(result, 114);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let sequences: Sequences = parse(&input);
        let result = pt2(&sequences);
        assert_eq!(result, 2);
    }
}
