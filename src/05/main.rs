use std::collections::HashSet;
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
    println!("Part 2: {}", pt2(&parse));
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

fn pt2(parse: &Parse) -> usize {
    // Create seed ranges
    let mut seed_ranges: Vec<Range<usize>> = Vec::new();
    parse.seeds.iter().enumerate().for_each(|(i, seed)| {
        if i % 2 == 0 {
            seed_ranges.push(*seed..seed + parse.seeds[i + 1]);
        };
    });

    // Slice seeds that are available in seed to soil map
    let mut sliced_seed_ranges: HashSet<Range<usize>> = HashSet::new();
    parse.maps[0].iter().for_each(|(_, source_range)| {
        seed_ranges.iter().for_each(|seed_range| {
            let mut range = seed_range.to_owned();
            if source_range.contains(&range.start) && source_range.contains(&range.end) {
                if source_range.start > range.start {
                    range.start = source_range.start;
                }
                if source_range.end < range.end {
                    range.end = source_range.end;
                }
                sliced_seed_ranges.insert(range);
            }
        });
    });

    // Calc lowest location
    let mut lowest_location: usize = 0;
    for seed_range in sliced_seed_ranges {
        for seed in seed_range {
            let mut source = seed as usize;
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
        }
    }

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

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let maps = parse(&input);
        let result = pt2(&maps);
        assert_eq!(result, 46);
    }
}
