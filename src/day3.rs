//! Day 3 - Gear ratios

use regex::Regex;
use std::collections::HashMap;

pub type Schematic = HashMap<(usize, usize), (u8, Vec<u32>)>;

pub fn parse_schematic(input: &str) -> Schematic {
    // First collect the location of all of the symbols in the input
    let schematic = input.lines().enumerate().fold(Default::default(), |mut schematic, (row, line)| {
        // Collect the location of all of the symbols on this line
        line.as_bytes().iter().enumerate()
            .filter_map(|(col, &ch)| {
                if ch.is_ascii_punctuation() && ch != b'.' {
                    Some(((row, col), (ch, vec![])))
                } else {
                    None
                }
            })
            .collect_into(&mut schematic);

        schematic
    });

    // Now scan for all of the numbers and see what symbols they're near
    input.lines().enumerate().fold(schematic, |mut schematic, (row, line)| {
        let re = Regex::new(r"\d+").expect("Should have been a valid regex");
        for needle in re.find_iter(line) {
            let num: u32 = needle.as_str().parse().expect("Should have been a valid number");

            // Figure out the edges of the number - these are the locations where a symbol
            // is interesting
            for j in row.saturating_sub(1)..=row+1 {
                for k in needle.start().saturating_sub(1)..=needle.end() {
                    schematic.entry((j, k)).and_modify(|c| c.1.push(num));
                }
            }
        }

        schematic
    })
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 4361; "with sample data")]
    #[test_case(PERSONAL_INPUT => 536202; "with real data")]
    pub fn problem1(input: &str) -> u32 {
        parse_schematic(input).values()
            .flat_map(|symbol| symbol.1.iter()).sum()
    }

    #[test_case(SAMPLE_INPUT => 467835; "with sample data")]
    #[test_case(PERSONAL_INPUT => 78272573; "with real data")]
    pub fn problem2(input: &str) -> u32 {
        parse_schematic(input).values()
            .filter(|&symbol| symbol.0 == b'*' && symbol.1.len() == 2)
            .map(|gear| gear.1[0] * gear.1[1])
            .sum()
    }

    const SAMPLE_INPUT: &str = include_str!("./input/day3-sample.txt");
    const PERSONAL_INPUT: &str = include_str!("./input/day3-real.txt");
}
