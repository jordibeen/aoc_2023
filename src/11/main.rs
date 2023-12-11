use std::time::Instant;

type Grid = Vec<Vec<char>>;
type Coords = (usize, usize);

fn main() {
    println!("--- Day 11: Cosmic Expansion ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut initial_galaxies: Vec<Coords> = Vec::new();
    let initial_grid: Grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '#' {
                        initial_galaxies.push((y, x));
                    }
                    char
                })
                .collect()
        })
        .collect();

    let (_, galaxies) = expand_grid(&initial_grid, &mut initial_galaxies);

    let sum: usize = galaxies.iter().fold(0, |acc, galaxy| {
        let distances: Vec<usize> = galaxies
            .iter()
            .map(|next_galaxy| {
                ((galaxy.0 as isize - next_galaxy.0 as isize).abs()
                    + (galaxy.1 as isize - next_galaxy.1 as isize).abs()) as usize
            })
            .collect();

        acc + distances.iter().sum::<usize>()
    });

    sum / 2
}

fn expand_grid(grid: &Grid, galaxies: &mut Vec<Coords>) -> (Grid, Vec<Coords>) {
    let mut expanded_grid: Grid = grid.clone();

    // Expand rows
    grid.iter().enumerate().for_each(|(y, row)| {
        if row.iter().all(|char| char == &'.') {
            let expansion_index = y + (expanded_grid.len() - grid.len());

            expanded_grid.insert(expansion_index, row.to_owned());

            galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.0 > expansion_index {
                    galaxy.0 += 1;
                }
            });
        }
    });

    // Expand columns
    (0..grid[0].len()).for_each(|x| {
        let col: Vec<char> = (0..grid.len()).map(|y| grid[y][x]).collect();
        if col.iter().all(|char| char == &'.') {
            let expansion_index = x + (expanded_grid[0].len() - grid[0].len());

            grid.iter().enumerate().for_each(|(y, _)| {
                expanded_grid[y].insert(expansion_index, '.');
            });

            galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.1 > expansion_index {
                    galaxy.1 += 1;
                }
            });
        }
    });

    (expanded_grid, galaxies.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 374);
    }
}
