//! Day 5 - If You Give A Seed A Fertilizer

pub struct MapEntry {
    dst: u64,
    src: u64,
    len: u64,
}

pub struct Map {
    entries: Vec<MapEntry>,
}

impl From<Vec<MapEntry>> for Map {
    fn from(value: Vec<MapEntry>) -> Self {
        Self { entries: value }
    }
}

impl Map {
    pub fn process(&self, input: u64) -> u64 {
        self.entries.iter()
            .find_map(|entry| {
                if input >= entry.src && input < entry.src + entry.len {
                    Some(input + entry.dst - entry.src)
                } else {
                    None
                }
            })
            .or(Some(input))
            .expect("Should always map to something")
    }

    pub fn process_ranges(&self, mut input: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut processed = vec![];

        for entry in &self.entries {
            let mut pending = vec![];

            while let Some((start, end)) = input.pop() {
                let preceeding = (start, end.min(entry.src));
                let overlap = (start.max(entry.src), end.min(entry.src + entry.len));
                let trailing = (start.max(entry.src + entry.len), end);

                if preceeding.1 > preceeding.0 {
                    pending.push(preceeding);
                }

                if overlap.1 > overlap.0 {
                    processed.push((overlap.0 + entry.dst - entry.src, overlap.1 + entry.dst - entry.src));
                }

                if trailing.1 > trailing.0 {
                    pending.push(trailing);
                }
            }

            input = pending;
        }

        processed.extend(input);
        processed
    }
}

pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> Self {
        let blocks: Vec<&str> = input.split("\n\n").collect();

        // First extract the seeds from the opening line
        let seeds = blocks[0]
            .split_once(':').expect("Should have started with 'seeds:'")
            .1
            .split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect();

        // Then each of the rest of the blocks is a map
        let maps = blocks[1..].iter()
            .map(|&block| {
                let entries = block.lines()
                    .skip(1)
                    .map(|line| line.split_whitespace().flat_map(str::parse::<u64>).collect::<Vec<_>>())
                    .map(|nums| MapEntry{ dst: nums[0], src: nums[1], len: nums[2] })
                    .collect::<Vec<_>>();
                Map::from(entries)
            })
        .collect();

        Self { seeds, maps }
    }

    pub fn process(&self, input: u64) -> u64 {
        self.maps.iter().fold(input, |accum, x| {
            x.process(accum)
        })
    }

    pub fn process_ranges(&self, input: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        self.maps.iter().fold(input, |accum, x| {
            x.process_ranges(accum)
        })
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 35; "with sample data")]
    #[test_case(PERSONAL_INPUT => 551761867; "with personal data")]
    pub fn problem1(input: &str) -> u64 {
        let almanac = Almanac::parse(input);

        almanac.seeds.iter().map(|seed| almanac.process(*seed))
            .min()
            .expect("Should have mapped to something")
    }

    #[test_case(SAMPLE_INPUT => 46; "with sample data")]
    #[test_case(PERSONAL_INPUT => 57451709; "with real data")]
    pub fn problem2(input: &str) -> u64 {
        let almanac = Almanac::parse(input);
        let seed_ranges = almanac.seeds.iter()
            .array_chunks::<2>()
            .map(|[&start, &len]| (start, start+len))
            .collect::<Vec<_>>();

        let mut x = almanac.process_ranges(seed_ranges);
        x.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        x[0].0
    }

    const SAMPLE_INPUT: &str = 
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    const PERSONAL_INPUT: &str = include_str!("./input/day5.txt");
}
