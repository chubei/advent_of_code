use std::ops::Range;

struct Map {
    entries: Vec<Entry>,
}

impl Map {
    fn get(&self, source: u64) -> u64 {
        for entry in &self.entries {
            if let Some(destination) = entry.get(source) {
                return destination;
            }
        }
        source
    }

    fn parse(input: &str) -> Map {
        let mut entries = vec![];
        for line in input.trim().lines() {
            if line.is_empty() {
                break;
            }
            entries.push(Entry::parse(line));
        }
        Map { entries }
    }
}

struct Entry {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Entry {
    fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.length
    }

    fn get(&self, source: u64) -> Option<u64> {
        if self.source_range().contains(&source) {
            Some(source - self.source_start + self.destination_start)
        } else {
            None
        }
    }

    // Parse "destination_start source_start length"
    fn parse(line: &str) -> Entry {
        let mut parts = line.split_whitespace();
        let destination_start = parts.next().unwrap().parse().unwrap();
        let source_start = parts.next().unwrap().parse().unwrap();
        let length = parts.next().unwrap().parse().unwrap();
        Entry {
            destination_start,
            source_start,
            length,
        }
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut parts = INPUT.split(':');
    parts.next().unwrap();
    let seeds = parts.next().unwrap().lines().next().unwrap().trim();
    let seeds: Vec<u64> = seeds.split(' ').map(|s| s.parse().unwrap()).collect();
    let seeds = seeds.chunks_exact(2);
    let seed_to_soil = Map::parse(parts.next().unwrap());
    let soil_to_fertilizer = Map::parse(parts.next().unwrap());
    let fertilizer_to_water = Map::parse(parts.next().unwrap());
    let water_to_light = Map::parse(parts.next().unwrap());
    let light_to_temperature = Map::parse(parts.next().unwrap());
    let temperature_to_humidity = Map::parse(parts.next().unwrap());
    let humidity_to_location = Map::parse(parts.next().unwrap());

    let mut result = u64::MAX;
    for seed in seeds {
        let start = seed[0];
        let length = seed[1];
        for seed in start..start + length {
            let soil = seed_to_soil.get(seed);
            let fertilizer = soil_to_fertilizer.get(soil);
            let water = fertilizer_to_water.get(fertilizer);
            let light = water_to_light.get(water);
            let temperature = light_to_temperature.get(light);
            let humidity = temperature_to_humidity.get(temperature);
            let location = humidity_to_location.get(humidity);
            result = result.min(location);
        }
    }
    println!("{}", result);
}
