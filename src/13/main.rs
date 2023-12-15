use std::time::Instant;

type Mirror = Vec<Vec<char>>;
type Mirrors = Vec<Mirror>;

fn main() {
    println!("--- Day 13: Point of Incidence ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    let (horizontal_mirrors, vertical_mirrors) = parse(&input);
    println!("Part 1: {}", pt1(&horizontal_mirrors, &vertical_mirrors));
    println!("Part 2: {}", pt2(&horizontal_mirrors, &vertical_mirrors));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn parse(input: &str) -> (Mirrors, Mirrors) {
    let mut horizontal_mirrors: Mirrors = Vec::new();
    let mut vertical_mirrors: Mirrors = Vec::new();

    let mut horizontal_mirror: Mirror = Vec::new();
    input.split("\n").for_each(|line| {
        if line.is_empty() {
            horizontal_mirrors.push(horizontal_mirror.to_owned());

            let mut vertical_mirror: Mirror =
                (0..horizontal_mirror[0].len()).map(|_| vec![]).collect();
            horizontal_mirror.iter().for_each(|row| {
                row.iter()
                    .enumerate()
                    .for_each(|(i, c)| vertical_mirror[i].push(*c));
            });
            vertical_mirrors.push(vertical_mirror);

            horizontal_mirror = Vec::new();
        } else {
            horizontal_mirror.push(line.chars().collect());
        }
    });

    (horizontal_mirrors, vertical_mirrors)
}

fn pt1(horizontal_mirrors: &Mirrors, vertical_mirrors: &Mirrors) -> usize {
    let horizontal_reflections: usize = horizontal_mirrors.iter().fold(0, |acc, mirror| {
        acc + mirror
            .iter()
            .enumerate()
            .map(|(i, row)| {
                if i + 1 <= mirror.len() - 1 && row == &mirror[i + 1] {
                    count_reflections(mirror, i) * 100
                } else {
                    0
                }
            })
            .sum::<usize>()
    });

    let vertical_reflections: usize = vertical_mirrors.iter().fold(0, |acc, mirror| {
        acc + mirror
            .iter()
            .enumerate()
            .map(|(i, col)| {
                if i + 1 <= mirror.len() - 1 && col == &mirror[i + 1] {
                    count_reflections(mirror, i)
                } else {
                    0
                }
            })
            .sum::<usize>()
    });

    horizontal_reflections + vertical_reflections
}

fn pt2(horizontal_mirrors: &Mirrors, vertical_mirrors: &Mirrors) -> usize {
    let horizontal_reflections: usize = horizontal_mirrors.iter().fold(0, |acc, mirror| {
        acc + mirror
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut comparisons: Vec<Vec<bool>> = Vec::new();
                if i + 1 <= mirror.len() - 1 {
                    mirror[..(i + 1)]
                        .iter()
                        .rev()
                        .zip(mirror[(i + 1)..].iter())
                        .for_each(|(left_row, right_row)| {
                            let comparison: Vec<bool> = left_row
                                .iter()
                                .zip(right_row)
                                .map(|(l, r)| l == r)
                                .collect();
                            comparisons.push(comparison.to_owned());
                        });
                }
                if comparisons
                    .iter()
                    .flatten()
                    .filter(|x| x == &&false)
                    .count()
                    == 1
                {
                    (i + 1) * 100
                } else {
                    0
                }
            })
            .sum::<usize>()
    });

    let vertical_reflections: usize = vertical_mirrors.iter().fold(0, |acc, mirror| {
        acc + mirror
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut comparisons: Vec<Vec<bool>> = Vec::new();
                if i + 1 <= mirror.len() - 1 {
                    mirror[..(i + 1)]
                        .iter()
                        .rev()
                        .zip(mirror[(i + 1)..].iter())
                        .for_each(|(left_row, right_row)| {
                            let comparison: Vec<bool> = left_row
                                .iter()
                                .zip(right_row)
                                .map(|(l, r)| l == r)
                                .collect();
                            comparisons.push(comparison.to_owned());
                        });
                }
                if comparisons
                    .iter()
                    .flatten()
                    .filter(|x| x == &&false)
                    .count()
                    == 1
                {
                    i + 1
                } else {
                    0
                }
            })
            .sum::<usize>()
    });

    horizontal_reflections + vertical_reflections
}

fn count_reflections(mirror: &Vec<Vec<char>>, i: usize) -> usize {
    let reflections: Vec<bool> = mirror[..(i + 1)]
        .iter()
        .rev()
        .zip(mirror[(i + 1)..].iter())
        .map(|(l, r)| l == r)
        .collect();

    if reflections.iter().all(|x| x == &true) {
        return i + 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (horizontal_mirrors, vertical_mirrors) = parse(&input);
        let result = pt1(&horizontal_mirrors, &vertical_mirrors);
        assert_eq!(result, 405);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (horizontal_mirrors, vertical_mirrors) = parse(&input);
        let result = pt2(&horizontal_mirrors, &vertical_mirrors);
        assert_eq!(result, 400);
    }
}
