use std::{collections::HashMap, time::Instant};

type Coords = (usize, usize);
type Grid = Vec<Vec<char>>;
type Occupations = HashMap<Coords, char>;
type Rocks = Vec<Coords>;

fn main() {
    println!("--- Day 14: Parabolic Reflector Dish ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut occupations: Occupations = HashMap::new();
    let mut rocks: Rocks = Vec::new();

    let grid: Grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'O' {
                        rocks.push((y, x));
                    }
                    occupations.insert((y, x), char);
                    char
                })
                .collect()
        })
        .collect();

    rocks.iter().for_each(|rock| {
        let mut y = rock.0;
        let mut rolling = true;

        while rolling {
            if y == 0 {
                rolling = false;
                continue;
            }
            if let Some(occupation) = occupations.get_mut(&(y - 1, rock.1)) {
                if occupation == &'.' {
                    *occupation = 'O';
                    occupations.insert((y, rock.1), '.');
                } else {
                    rolling = false;
                }
            }
            y -= 1;
        }
    });

    // Debug grid
    // println!("----------");
    // grid.iter().enumerate().for_each(|(y, row)| {
    //     row.iter().enumerate().for_each(|(x, _)| {
    //         print!("{}", occupations.get(&(y, x)).unwrap());
    //     });
    //     print!("\n");
    // });

    let sum: usize = grid.iter().enumerate().fold(0, |sum, (y, row)| {
        sum + row.iter().enumerate().fold(0, |acc, (x, _)| {
            if occupations.get(&(y, x)).unwrap() == &'O' {
                acc + (grid.len() - y)
            } else {
                acc
            }
        })
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
        assert_eq!(result, 136);
    }
}
