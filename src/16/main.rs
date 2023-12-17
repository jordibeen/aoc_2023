use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

type Coords = (isize, isize);
type Direction = (isize, isize);
type Beam = (Coords, Direction);

fn main() {
    println!("--- Day 16: The Floor Will Be Lava ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut occupations: HashMap<Coords, char> = HashMap::new();
    let mut energized: Vec<Coords> = Vec::new();

    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    occupations.insert((y as isize, x as isize), char);
                    char
                })
                .collect()
        })
        .collect();

    // Debug grid
    println!("=============");
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            print!("{}", occupations.get(&(y as isize, x as isize)).unwrap());
        });
        print!("\n");
    });
    println!("=============");

    let mut beams: VecDeque<Beam> = VecDeque::new();
    let mut checked_beams: Vec<Beam> = Vec::new();

    beams.push_front(((0, -1), (0, 1)));

    while let Some(beam) = beams.pop_front() {
        if checked_beams.contains(&beam) {
            continue;
        }

        let loc = beam.0;
        let direction = beam.1;

        if !energized.contains(&loc) {
            energized.push(loc);
        }

        let next_loc = ((loc.0 + direction.0), (loc.1 + direction.1));

        if let Some(character) = occupations.get(&next_loc) {
            match character {
                '.' => {
                    beams.push_back((next_loc, direction));
                }
                '/' => {
                    if direction.0 == 1 {
                        beams.push_back((next_loc, (0, -1)));
                    }
                    if direction.0 == -1 {
                        beams.push_back((next_loc, (0, 1)));
                    }
                    if direction.1 == 1 {
                        beams.push_back((next_loc, (-1, 0)));
                    }
                    if direction.1 == -1 {
                        beams.push_back((next_loc, (1, 0)));
                    }
                }
                '\\' => {
                    if direction.0 == 1 {
                        beams.push_back((next_loc, (0, 1)));
                    }
                    if direction.0 == -1 {
                        beams.push_back((next_loc, (0, -1)));
                    }
                    if direction.1 == 1 {
                        beams.push_back((next_loc, (1, 0)));
                    }
                    if direction.1 == -1 {
                        beams.push_back((next_loc, (-1, 0)));
                    }
                }
                '|' => {
                    if direction.1 != 0 {
                        beams.push_back((next_loc, (-1, 0)));
                        beams.push_back((next_loc, (1, 0)));
                    } else {
                        beams.push_back((next_loc, direction));
                    }
                }
                '-' => {
                    if direction.0 != 0 {
                        beams.push_back((next_loc, (0, -1)));
                        beams.push_back((next_loc, (0, 1)));
                    } else {
                        beams.push_back((next_loc, direction));
                    }
                }
                _ => (),
            }
        }

        checked_beams.push(beam);
    }

    println!("Energized");
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            // print!("{}", occupations.get(&(y, x)).unwrap());
            if energized.contains(&(y as isize, x as isize)) {
                print!("#");
            } else {
                print!(".");
            }
        });
        print!("\n");
    });
    println!("=============");

    energized.iter().count() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 46);
    }
}
