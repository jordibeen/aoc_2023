use std::ops::Range;
use std::time::Instant;

#[derive(Debug)]
struct Parse {
    seeds: Vec<usize>,
    maps: Vec<Vec<(Range<usize>, Range<usize>)>>,
}

impl Parse {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            maps: Vec::new(),
        }
    }
}

fn main() {
    println!("--- Day 5: If You Give A Seed A Fertilizer ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    let parse: Parse = parse(&input);
    println!("Part 1: {}", pt1(&parse));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn parse(input: &str) -> Parse {
    let mut parse: Parse = Parse::new();

    input.lines().enumerate().for_each(|(i, line)| {
        // Parse seeds
        if i == 0 {
            parse.seeds = line
                .split_once(": ")
                .unwrap()
                .1
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
        }

        // Parse Map Type
        if line.contains("map:") {
            parse.maps.push(Vec::new());
        }

        // Parse ranges
        let info: Vec<_> = line.split(" ").collect();
        if info.len() == 3 {
            let (destination_range_start, source_range_start, range_length) = (
                info[0].parse::<usize>().unwrap(),
                info[1].parse::<usize>().unwrap(),
                info[2].parse::<usize>().unwrap(),
            );

            parse.maps.last_mut().unwrap().push((
                destination_range_start..(destination_range_start + range_length),
                source_range_start..(source_range_start + range_length),
            ));
        }
    });

    parse
}

fn pt1(parse: &Parse) -> usize {
    let mut lowest_location = 0;
    parse.seeds.iter().for_each(|seed| {
        let mut source = *seed;
        let mut destination = source;

        parse.maps.iter().for_each(|map| {
            map.iter().for_each(|(destination_range, source_range)| {
                if source_range.contains(&source) {
                    destination = destination_range.start + (source - source_range.start);
                }
            });
            source = destination;
        });

        if lowest_location == 0 || destination < lowest_location {
            lowest_location = destination;
        }
    });

    lowest_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let parse = parse(&input);
        let result = pt1(&parse);
        assert_eq!(result, 35);
    }
}
