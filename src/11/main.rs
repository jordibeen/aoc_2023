use std::time::Instant;

type Coords = (usize, usize);
type Grid = Vec<Vec<char>>;
type Galaxies = Vec<Coords>;

fn main() {
    println!("--- Day 11: Cosmic Expansion ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();

    let (grid, galaxies): (Grid, Galaxies) = parse(&input);
    println!("Part 1: {}", expand_and_calc(&grid, &galaxies, 2));
    println!("Part 2: {}", expand_and_calc(&grid, &galaxies, 1000000));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn parse(input: &str) -> (Grid, Galaxies) {
    let mut galaxies: Galaxies = Vec::new();
    let grid: Grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '#' {
                        galaxies.push((y, x));
                    }
                    char
                })
                .collect()
        })
        .collect();

    (grid, galaxies)
}

fn expand_and_calc(grid: &Grid, galaxies: &Galaxies, times: usize) -> usize {
    let mut expanded_grid: Grid = grid.clone();
    let mut galaxies: Galaxies = galaxies.clone();

    // Expand rows
    grid.iter().enumerate().for_each(|(y, row)| {
        if row.iter().all(|char| char == &'.') {
            (1..times).for_each(|_| {
                let expansion_index = y + (expanded_grid.len() - grid.len());

                expanded_grid.insert(expansion_index, row.to_owned());

                galaxies.iter_mut().for_each(|galaxy| {
                    if galaxy.0 > expansion_index {
                        galaxy.0 += 1;
                    }
                });
            })
        }
    });

    // Expand columns
    (0..grid[0].len()).for_each(|x| {
        let col: Vec<char> = (0..grid.len()).map(|y| grid[y][x]).collect();
        if col.iter().all(|char| char == &'.') {
            (1..times).for_each(|_| {
                let expansion_index = x + (expanded_grid[0].len() - grid[0].len());

                grid.iter().enumerate().for_each(|(y, _)| {
                    expanded_grid[y].insert(expansion_index, '.');
                });

                galaxies.iter_mut().for_each(|galaxy| {
                    if galaxy.1 > expansion_index {
                        galaxy.1 += 1;
                    }
                });
            });
        }
    });

    // Sum distances between galaxies
    let sum: usize = galaxies.iter().fold(0, |acc, l| {
        let distances: Vec<usize> = galaxies
            .iter()
            .map(|r| {
                ((l.0 as isize - r.0 as isize).abs() + (l.1 as isize - r.1 as isize).abs()) as usize
            })
            .collect();

        acc + distances.iter().sum::<usize>()
    });

    sum / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (grid, galaxies) = parse(&input);
        let result = expand_and_calc(&grid, &galaxies, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (grid, galaxies) = parse(&input);
        let result = expand_and_calc(&grid, &galaxies, 10);
        assert_eq!(result, 1030);
        let result = expand_and_calc(&grid, &galaxies, 100);
        assert_eq!(result, 8410);
    }
}
