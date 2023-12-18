use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

type Coords = (isize, isize);
type Direction = (isize, isize);
type Beam = (Coords, Direction);
type Occupations = HashMap<Coords, char>;
type Grid = Vec<Vec<char>>;

fn main() {
    println!("--- Day 16: The Floor Will Be Lava ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();

    let (occupations, grid) = parse(&input);
    println!("Part 1: {}", pt1(&occupations));
    println!("Part 2: {}", pt2(&occupations, &grid));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn parse(input: &str) -> (Occupations, Grid) {
    let mut occupations: Occupations = HashMap::new();

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

    (occupations, grid)
}

fn pt1(occupations: &Occupations) -> usize {
    enlighten(&occupations, ((0, -1), (0, 1)))
}

fn pt2(occupations: &Occupations, grid: &Grid) -> usize {
    let mut initial_beams: HashMap<Beam, usize> = HashMap::new();

    let max_y = grid.len();
    let max_x = grid[0].len();
    (0..max_y).for_each(|y| {
        initial_beams.insert(((y as isize, -1), (0, 1)), 0);
        initial_beams.insert(((y as isize, max_x as isize), (0, -1)), 0);
    });
    (0..max_x).for_each(|x| {
        initial_beams.insert(((-1, x as isize), (1, 0)), 0);
        initial_beams.insert(((max_y as isize, x as isize), (-1, 0)), 0);
    });

    initial_beams.iter_mut().for_each(|(initial_beam, score)| {
        *score = enlighten(&occupations, *initial_beam);
    });

    *initial_beams.values().max().unwrap()
}

fn enlighten(occupations: &Occupations, starting_beam: Beam) -> usize {
    let mut energized: Vec<Coords> = Vec::new();
    let mut beams: VecDeque<Beam> = VecDeque::new();
    let mut checked_beams: Vec<Beam> = Vec::new();

    beams.push_front(starting_beam);

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
    energized.iter().count() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (occupations, _) = parse(&input);
        let result = pt1(&occupations);
        assert_eq!(result, 46);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (occupations, grid) = parse(&input);
        let result = pt2(&occupations, &grid);
        assert_eq!(result, 51);
    }
}
