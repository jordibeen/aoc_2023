use std::{collections::HashMap, time::Instant};

fn main() {
    println!("--- Day 19: Aplenty ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (workflow_section, parts_section) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, Vec<&str>> = workflow_section
        .split("\n")
        .map(|w| {
            let s = w.split_once("{").unwrap();
            (s.0, s.1[0..s.1.len() - 1].split(",").collect())
        })
        .collect();

    let sum = parts_section
        .split("\n")
        .filter(|x| !x.is_empty())
        .fold(0, |acc, p| {
            let parts: Vec<i32> = p[1..p.len() - 1]
                .split(",")
                .map(|part| part.split_once("=").unwrap().1.parse::<i32>().unwrap())
                .collect();

            let mut curr = "in";
            while !["A", "R"].contains(&curr) {
                for rule in workflows.get(&curr).unwrap() {
                    if let Some((specs, res)) = rule.split_once(":") {
                        let part_val = match &specs[0..1] {
                            "x" => &parts[0],
                            "m" => &parts[1],
                            "a" => &parts[2],
                            "s" => &parts[3],
                            _ => unreachable!(),
                        };

                        let rule_val: &i32 = &specs[2..].parse::<i32>().unwrap();
                        let fit = match &specs[1..2] {
                            ">" => part_val > rule_val,
                            "<" => part_val < rule_val,
                            _ => unreachable!(),
                        };

                        if fit {
                            curr = res;
                            break;
                        }
                    } else {
                        curr = rule;
                        break;
                    }
                }
            }

            match curr {
                "A" => acc + parts.iter().sum::<i32>(),
                "R" => acc,
                _ => unreachable!(),
            }
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
        assert_eq!(result, 19114);
    }
}
