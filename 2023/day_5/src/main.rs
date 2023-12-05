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

    fn get_range(&self, source: Range<u64>) -> Vec<Range<u64>> {
        let mut unmapped = vec![source];
        let mut mapped = vec![];
        for entry in &self.entries {
            let mut new_unmapped = vec![];
            for source in unmapped {
                let (entry_new_unmapped, entry_mapped) = entry.get_range(source);
                new_unmapped.extend(entry_new_unmapped);
                if let Some(entry_mapped) = entry_mapped {
                    mapped.push(entry_mapped);
                }
            }
            unmapped = new_unmapped;
        }
        mapped.extend(unmapped);
        mapped
    }

    fn get_all_ranges(&self, sources: Vec<Range<u64>>) -> Vec<Range<u64>> {
        sources
            .into_iter()
            .map(|source| self.get_range(source))
            .flatten()
            .collect()
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

    fn get_range(&self, source: Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
        if self.source_range().contains(&source.start)
            && self.source_range().contains(&(source.end - 1))
        {
            // source is within source range
            let destination_start = source.start - self.source_start + self.destination_start;
            let destination_end = source.end - self.source_start + self.destination_start;
            (vec![], Some(destination_start..destination_end))
        } else if self.source_range().contains(&source.start)
            && source.end > self.source_range().end
        {
            // source first half is within source range
            let destination_start = source.start - self.source_start + self.destination_start;
            let destination_end = self.destination_start + self.length;
            (
                vec![self.source_range().end..source.end],
                Some(destination_start..destination_end),
            )
        } else if source.start < self.source_start
            && self.source_range().contains(&(source.end - 1))
        {
            // source second half is within source range
            let destination_start = self.destination_start;
            let destination_end = source.end - self.source_start + self.destination_start;
            (
                vec![source.start..self.source_start],
                Some(destination_start..destination_end),
            )
        } else if source.start < self.source_start && source.end > self.source_range().end {
            // source contains source range
            let destination_start = self.destination_start;
            let destination_end = self.destination_start + self.length;
            (
                vec![
                    source.start..self.source_start,
                    self.source_range().end..source.end,
                ],
                Some(destination_start..destination_end),
            )
        } else {
            // no intersection
            (vec![source], None)
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
    let seeds = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    let seed_to_soil = Map::parse(parts.next().unwrap());
    let soil_to_fertilizer = Map::parse(parts.next().unwrap());
    let fertilizer_to_water = Map::parse(parts.next().unwrap());
    let water_to_light = Map::parse(parts.next().unwrap());
    let light_to_temperature = Map::parse(parts.next().unwrap());
    let temperature_to_humidity = Map::parse(parts.next().unwrap());
    let humidity_to_location = Map::parse(parts.next().unwrap());

    let soils = seed_to_soil.get_all_ranges(seeds);
    let fertilizers = soil_to_fertilizer.get_all_ranges(soils);
    let waters = fertilizer_to_water.get_all_ranges(fertilizers);
    let lights = water_to_light.get_all_ranges(waters);
    let temperatures = light_to_temperature.get_all_ranges(lights);
    let humidities = temperature_to_humidity.get_all_ranges(temperatures);
    let locations = humidity_to_location.get_all_ranges(humidities);
    println!("{}", locations.iter().map(|range| range.start).min().unwrap());
}
