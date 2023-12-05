//! Day 5 - If You Give A Seed A Fertilizer

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapEntry {
    dst: u64,
    src: u64,
    len: u64,
}

impl MapEntry {
    pub fn new(dst: u64, src: u64, len: u64) -> Self {
        Self { dst, src, len }
    }

    pub fn process(&self, input: u64) -> Option<u64> {
        if input >= self.src && input < self.src + self.len {
            Some(input+self.dst-self.src)
        } else {
            None
        }
    }

    pub fn reverse(&self, input: u64) -> Option<u64> {
        if input >= self.dst && input < self.dst + self.len {
            Some(input+self.src-self.dst)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
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
            .find_map(|entry| entry.process(input))
            .or(Some(input))
            .expect("Should always map to something")
    }

    pub fn reverse(&self, input: u64) -> u64 {
        self.entries.iter()
            .find_map(|entry| entry.reverse(input))
            .or(Some(input))
            .expect("Should always map to something")
    }
}

#[derive(Clone, Debug, Default)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> Self {
        let mut almanac = Self::default();
        let blocks: Vec<&str> = input.split("\n\n").collect();

        // First extract the seeds from the opening line
        blocks[0]
            .split_once(':').expect("Should have started with 'seeds:'")
            .1
            .split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect_into(&mut almanac.seeds);

        // Then each of the rest of the blocks is a map
        blocks[1..].iter()
            .map(|&block| {
                let entries = block.lines()
                    .skip(1)
                    .map(|line| line.split_whitespace().flat_map(str::parse::<u64>).collect::<Vec<_>>())
                    .map(|nums| MapEntry::new(nums[0], nums[1], nums[2]))
                    .collect::<Vec<_>>();
                Map::from(entries)
            })
        .collect_into(&mut almanac.maps);

        almanac
    }

    pub fn process(&self, input: u64) -> u64 {
        self.maps.iter().fold(input, |accum, x| {
            x.process(accum)
        })
    }

    pub fn reverse(&self, input: u64) -> u64 {
        self.maps.iter().rev().fold(input, |accum, x| {
            x.reverse(accum)
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

    #[test_case(SAMPLE_INPUT, 35 => 13; "what seed in location 35")]
    #[test_case(SAMPLE_INPUT, 86 => 55; "what seed in location 86")]
    #[test_case(SAMPLE_INPUT, 43 => 14; "what seed in location 43")]
    #[test_case(SAMPLE_INPUT, 82 => 79; "what seed in location 82")]
    pub fn validate_reverse(input: &str, location: u64) -> u64 {
        let almanac = Almanac::parse(input);
        almanac.reverse(location)
    }

    #[test_case(SAMPLE_INPUT => 46; "with sample data")]
    #[test_case(PERSONAL_INPUT => 57451709; "with real data")]
    pub fn problem2(input: &str) -> u64 {
        let almanac = Almanac::parse(input);
        let seed_ranges = almanac.seeds.iter()
            .array_chunks::<2>()
            .map(|[&start, &len]| (start..start+len))
            .collect::<Vec<_>>();

        for candidate in 0.. {
            let seed_for_loc = almanac.reverse(candidate);
            if seed_ranges.iter().any(|elem| elem.contains(&seed_for_loc)) {
                return candidate;
            }
        }

        unreachable!("All that work for nothing");
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
