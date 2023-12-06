use std::time::Instant;

fn main() {
    println!("--- Day 6: Wait For It ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> u32 {
    let (times_s, distances_s) = input.split_once('\n').unwrap();
    let times: Vec<u16> = times_s
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|x| x.parse::<u16>().ok())
        .collect();
    let distances: Vec<u16> = distances_s
        .trim_end()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|x| x.parse::<u16>().ok())
        .collect();

    let mul: u32 = (0..times.len()).fold(0, |mul, race| {
        let race_time: u16 = times[race];
        let race_record: u16 = distances[race];

        let ways_to_beat: u32 = (0..race_time).fold(0, |ways, holding_seconds| {
            if (holding_seconds * 1) * (race_time - holding_seconds) > race_record {
                ways + 1
            } else {
                ways
            }
        });

        if mul == 0 {
            ways_to_beat
        } else {
            mul * ways_to_beat
        }
    });

    mul
}

fn pt2(input: &str) -> u64 {
    let (times_s, distances_s) = input.split_once('\n').unwrap();
    let race_time = times_s
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let race_record = distances_s
        .trim_end()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let ways_to_beat: u64 = (0..race_time).fold(0, |ways, holding_seconds| {
        if (holding_seconds * 1) * (race_time - holding_seconds) > race_record {
            ways + 1
        } else {
            ways
        }
    });

    ways_to_beat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 71503);
    }
}
