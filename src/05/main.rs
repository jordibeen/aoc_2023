use std::ops::Range;
use std::time::Instant;

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<(Range<usize>, Range<usize>)>,
    soil_to_fertilizer: Vec<(Range<usize>, Range<usize>)>,
    fertilizer_to_water: Vec<(Range<usize>, Range<usize>)>,
    water_to_light: Vec<(Range<usize>, Range<usize>)>,
    light_to_temperature: Vec<(Range<usize>, Range<usize>)>,
    temperature_to_humidity: Vec<(Range<usize>, Range<usize>)>,
    humidity_to_location: Vec<(Range<usize>, Range<usize>)>,
}

impl Maps {
    fn new() -> Self {
        Self {
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLigt,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    None,
}
//

fn main() {
    println!("--- Day 5: If You Give A Seed A Fertilizer ---");
    let input: &str = include_str!("./input.txt");
    let start: Instant = Instant::now();
    println!("Part 1: {}", pt1(&input));
    println!("Execution time: {:.3?}", start.elapsed());
}

fn pt1(input: &str) -> usize {
    let mut seeds: Vec<usize> = Vec::new();
    let mut selected_map: MapType = MapType::None;
    let mut maps: Maps = Maps::new();

    input.lines().enumerate().for_each(|(i, line)| {
        // Parse seeds
        if i == 0 {
            seeds = line
                .split_once(": ")
                .unwrap()
                .1
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
        }

        // Parse Map Type
        if line.contains("map:") {
            match line {
                "seed-to-soil map:" => selected_map = MapType::SeedToSoil,
                "soil-to-fertilizer map:" => selected_map = MapType::SoilToFertilizer,
                "fertilizer-to-water map:" => selected_map = MapType::FertilizerToWater,
                "water-to-light map:" => selected_map = MapType::WaterToLigt,
                "light-to-temperature map:" => selected_map = MapType::LightToTemperature,
                "temperature-to-humidity map:" => selected_map = MapType::TemperatureToHumidity,
                "humidity-to-location map:" => selected_map = MapType::HumidityToLocation,
                _ => selected_map = MapType::None,
            }
        }

        // Parse ranges
        let info: Vec<_> = line.split(" ").collect();
        if info.len() == 3 {
            let (destination_range_start, source_range_start, range_length) = (
                info[0].parse::<usize>().unwrap(),
                info[1].parse::<usize>().unwrap(),
                info[2].parse::<usize>().unwrap(),
            );

            let range_tup = (
                destination_range_start..(destination_range_start + range_length),
                source_range_start..(source_range_start + range_length),
            );

            match selected_map {
                MapType::SeedToSoil => {
                    maps.seed_to_soil.push(range_tup);
                }
                MapType::SoilToFertilizer => {
                    maps.soil_to_fertilizer.push(range_tup);
                }
                MapType::FertilizerToWater => {
                    maps.fertilizer_to_water.push(range_tup);
                }
                MapType::WaterToLigt => {
                    maps.water_to_light.push(range_tup);
                }
                MapType::LightToTemperature => {
                    maps.light_to_temperature.push(range_tup);
                }
                MapType::TemperatureToHumidity => {
                    maps.temperature_to_humidity.push(range_tup);
                }
                MapType::HumidityToLocation => {
                    maps.humidity_to_location.push(range_tup);
                }
                _ => (),
            }
        }
    });

    let mut lowest_location = 0;
    seeds.iter().for_each(|seed| {
        let mut soil = *seed;
        maps.seed_to_soil
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(seed) {
                    soil = destination_range.start + (seed - source_range.start);
                }
            });

        let mut fertilizer = soil;
        maps.soil_to_fertilizer
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(&soil) {
                    fertilizer = destination_range.start + (&soil - source_range.start);
                }
            });

        let mut water = fertilizer;
        maps.fertilizer_to_water
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(&fertilizer) {
                    water = destination_range.start + (&fertilizer - source_range.start);
                }
            });

        let mut light = water;
        maps.water_to_light
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(&water) {
                    light = destination_range.start + (&water - source_range.start);
                }
            });

        let mut temperature = light;
        maps.light_to_temperature
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(&light) {
                    temperature = destination_range.start + (&light - source_range.start);
                }
            });

        let mut humidity = temperature;
        maps.temperature_to_humidity
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(&temperature) {
                    humidity = destination_range.start + (&temperature - source_range.start);
                }
            });

        let mut location = humidity;
        maps.humidity_to_location
            .iter()
            .for_each(|(destination_range, source_range)| {
                if source_range.contains(&humidity) {
                    location = destination_range.start + (&humidity - source_range.start);
                }
            });

        if lowest_location == 0 || location < lowest_location {
            lowest_location = location;
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
        let result = pt1(&input);
        assert_eq!(result, 35);
    }
}
