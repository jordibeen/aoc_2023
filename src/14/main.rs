use std::{collections::HashMap, time::Instant};

type Coords = (usize, usize);
type Grid = Vec<Vec<char>>;
type Occupations = HashMap<Coords, char>;
type Rocks = Vec<Coords>;

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn main() {
    println!("--- Day 14: Parabolic Reflector Dish ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut occupations: &mut Occupations = &mut HashMap::new();
    let rocks: &mut Rocks = &mut Vec::new();

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

    (occupations, _) = tilt(occupations, rocks, 0, Direction::NORTH);

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

fn pt2(input: &str) -> usize {
    let mut occupations: &mut Occupations = &mut HashMap::new();
    let mut rocks: &mut Rocks = &mut Vec::new();

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

    let max_y: usize = grid.len();
    let max_x: usize = grid[0].len();

    let mut encountered_rocks: Vec<Rocks> = Vec::new();
    let mut cycle_repeat: usize = 0;
    let mut cycle_offset: usize = 0;
    for i in 0..1_000_000_000 {
        (occupations, rocks) = tilt(occupations, rocks, 0, Direction::NORTH);
        (occupations, rocks) = tilt(occupations, rocks, 0, Direction::WEST);
        (occupations, rocks) = tilt(occupations, rocks, max_y, Direction::SOUTH);
        (occupations, rocks) = tilt(occupations, rocks, max_x, Direction::EAST);

        if let Some(x) = encountered_rocks.iter().position(|x| x == rocks) {
            cycle_repeat = i;
            cycle_offset = x;
            break;
        } else {
            encountered_rocks.push(rocks.clone());
        }
    }

    let cycles_remaining = (1_000_000_000 - cycle_offset) % (cycle_repeat - cycle_offset) - 1;
    for _ in 0..cycles_remaining {
        (occupations, rocks) = tilt(occupations, rocks, 0, Direction::NORTH);
        (occupations, rocks) = tilt(occupations, rocks, 0, Direction::WEST);
        (occupations, rocks) = tilt(occupations, rocks, max_y, Direction::SOUTH);
        (occupations, rocks) = tilt(occupations, rocks, max_x, Direction::EAST);
    }

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

fn tilt<'a>(
    occupations: &'a mut Occupations,
    rocks: &'a mut Rocks,
    max: usize,
    direction: Direction,
) -> (&'a mut Occupations, &'a mut Rocks) {
    match direction {
        Direction::NORTH => {
            rocks.sort_by_key(|&(y, x)| (y, x));
            rocks.iter_mut().for_each(|rock| {
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
                            occupations.insert(*rock, '.');
                            rock.0 = y - 1;
                        } else {
                            rolling = false;
                        }
                    }
                    y -= 1;
                }
            });
        }
        Direction::WEST => {
            rocks.sort_by_key(|&(y, x)| (x, y));
            rocks.iter_mut().for_each(|rock| {
                let mut x = rock.1;
                let mut rolling = true;

                while rolling {
                    if x == 0 {
                        rolling = false;
                        continue;
                    }
                    if let Some(occupation) = occupations.get_mut(&(rock.0, x - 1)) {
                        if occupation == &'.' {
                            *occupation = 'O';
                            occupations.insert((rock.0, x), '.');
                            rock.1 = x - 1;
                        } else {
                            rolling = false;
                        }
                    }
                    x -= 1;
                }
            });
        }
        Direction::SOUTH => {
            rocks.sort_by_key(|&(y, x)| (!y, x));
            rocks.iter_mut().for_each(|rock| {
                let mut y = rock.0;
                let mut rolling = true;

                while rolling {
                    if y == max {
                        rolling = false;
                        continue;
                    }
                    if let Some(occupation) = occupations.get_mut(&(y + 1, rock.1)) {
                        if occupation == &'.' {
                            *occupation = 'O';
                            occupations.insert((y, rock.1), '.');
                            rock.0 = y + 1;
                        } else {
                            rolling = false;
                        }
                    }
                    y += 1;
                }
            });
        }
        Direction::EAST => {
            rocks.sort_by_key(|&(y, x)| (y, !x));
            rocks.iter_mut().for_each(|rock| {
                let mut x = rock.1;
                let mut rolling = true;

                while rolling {
                    if x == max {
                        rolling = false;
                        continue;
                    }
                    if let Some(occupation) = occupations.get_mut(&(rock.0, x + 1)) {
                        if occupation == &'.' {
                            *occupation = 'O';
                            occupations.insert((rock.0, x), '.');
                            rock.1 = x + 1;
                        } else {
                            rolling = false;
                        }
                    }
                    x += 1;
                }
            });
        }
    }

    (occupations, rocks)
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

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 64);
    }
}
