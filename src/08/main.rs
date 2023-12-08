use std::{collections::HashMap, time::Instant};

fn main() {
    println!("--- Day 2: Cube Conundrum ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (instructions, nodes) = input.split_once("\n").unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for node in nodes.trim().split("\n") {
        let (node_name, els) = node.split_once(" = ").unwrap();
        let elements = els.replace("(", "").replace(")", "");
        let (el1, el2) = elements.split_once(", ").unwrap();
        map.insert(node_name.to_owned(), (el1.to_owned(), el2.to_owned()));
    }

    let mut steps = 0;
    let mut cursor: String = "AAA".to_string();
    while cursor != "ZZZ" {
        for instruction in instructions.chars() {
            let (left, right) = map.get(&cursor).unwrap();
            if instruction == 'L' {
                cursor = left.to_owned();
            }
            if instruction == 'R' {
                cursor = right.to_owned();
            }
            steps += 1;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 2);
    }
}
