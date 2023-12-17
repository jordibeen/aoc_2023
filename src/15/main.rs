use std::time::Instant;

fn main() {
    println!("--- Day 15: Lens Library ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    input.split(",").map(|step| hash(step.trim())).sum()
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
}
