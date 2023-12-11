use std::time::Instant;

// const DIRECTIONS: [&'static str; 4] = ["north", "east", "south", "west"];
type Grid = Vec<Vec<char>>;
type Position = (isize, isize);

fn main() {
    println!("--- Day 10: Pipe Maze ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut route: Vec<Position> = Vec::new();
    let grid: Grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        route.push((y as isize, x as isize));
                    };
                    char
                })
                .collect::<Vec<char>>()
        })
        .collect();

    // Set current position
    let mut curr_pos: Position = route.first().unwrap().to_owned();

    // Get directions to go initally
    let initial_directions: Vec<Position> = get_initial_directions(&grid, &curr_pos);

    // Set previous position
    let mut prev_pos: Position = curr_pos.clone();

    // Try all directions possible from initial position
    for initial_direction in initial_directions {
        let mut next_pos: Position = initial_direction;

        // Recursively make steps until the initial position is reached again
        while next_pos != route[0] {
            // Move forward
            curr_pos = next_pos;
            if !route.contains(&curr_pos) {
                route.push(curr_pos);
            }

            // Determine next move
            let next_char: char = grid[next_pos.0 as usize][next_pos.1 as usize];
            next_pos = match next_char {
                '|' => {
                    // From north to south
                    if prev_pos.0 < curr_pos.0 {
                        (next_pos.0 + 1, next_pos.1)
                    // From south to north
                    } else {
                        (next_pos.0 - 1, next_pos.1)
                    }
                }
                '-' => {
                    // From west to east
                    if prev_pos.1 < curr_pos.1 {
                        (next_pos.0, next_pos.1 + 1)
                    // From east to west
                    } else {
                        (next_pos.0, next_pos.1 - 1)
                    }
                }
                'L' => {
                    // From north to east
                    if prev_pos.0 < curr_pos.0 {
                        (next_pos.0, next_pos.1 + 1)
                    // From east to north
                    } else {
                        (next_pos.0 - 1, next_pos.1)
                    }
                }
                'J' => {
                    // From north to west
                    if prev_pos.0 < curr_pos.0 {
                        (next_pos.0, next_pos.1 - 1)
                    // From west to north
                    } else {
                        (next_pos.0 - 1, next_pos.1)
                    }
                }
                '7' => {
                    // From south to west
                    if prev_pos.0 > curr_pos.0 {
                        (next_pos.0, next_pos.1 - 1)
                    // From west to south
                    } else {
                        (next_pos.0 + 1, next_pos.1)
                    }
                }
                'F' => {
                    // From south to east
                    if prev_pos.0 > curr_pos.0 {
                        (next_pos.0, next_pos.1 + 1)
                    // From east to south
                    } else {
                        (next_pos.0 + 1, next_pos.1)
                    }
                }
                '.' => {
                    panic!("found .");
                }
                _ => {
                    panic!("unhandled");
                }
            };

            // Keep track of previous move
            prev_pos = curr_pos;
        }
    }

    // // Draw to debug
    // for y in 0..grid.len() {
    //     for x in 0..grid[0].len() {
    //         if route.contains(&(y as isize, x as isize)) {
    //             print!("{}", grid[y as usize][x as usize]);
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }

    route.len() / 2
}

fn get_initial_directions(grid: &Grid, initial_pos: &Position) -> Vec<Position> {
    let mut initial_directions: Vec<Position> = Vec::new();

    if initial_pos.0 > 0 {
        let north_char = grid[initial_pos.0 as usize - 1][initial_pos.1 as usize];
        if north_char == '|' || north_char == '7' || north_char == 'F' {
            initial_directions.push((initial_pos.0 - 1, initial_pos.1));
        }
    };

    let east_char = grid[initial_pos.0 as usize][initial_pos.1 as usize + 1];
    if east_char == '-' || east_char == 'J' || east_char == '7' {
        initial_directions.push((initial_pos.0, initial_pos.1 + 1));
    };

    let south_char = grid[initial_pos.0 as usize + 1][initial_pos.1 as usize];
    if south_char == '|' || south_char == 'L' || south_char == 'J' {
        initial_directions.push((initial_pos.0 + 1, initial_pos.1));
    };

    if initial_pos.1 > 0 {
        let west_char = grid[initial_pos.0 as usize][initial_pos.1 as usize - 1];
        if west_char == '-' || west_char == 'L' || west_char == 'F' {
            initial_directions.push((initial_pos.0, initial_pos.1 - 1));
        }
    };

    initial_directions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1_1.txt");
        let result = pt1(&input);
        assert_eq!(result, 4);

        let input = include_str!("./example_pt1_2.txt");
        let result = pt1(&input);
        assert_eq!(result, 8);
    }
}
