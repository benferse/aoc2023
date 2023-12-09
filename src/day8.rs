//! Day 8 - Haunted Wasteland

use regex::Regex;
use std::collections::HashMap;

pub struct Map<'a> {
    instructions: &'a str,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    pub fn parse(input: &'a str) -> Self {
        // First line is the set of instructions, rest are node mappings
        let (instructions, rest) = input.split_once("\n\n").expect("Malformed");
        let re = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").expect("Should have compiled");
        let nodes = re.captures_iter(rest).map(|c| {
            let (_, [start, left, right]) = c.extract();
            (start, (left, right))
        })
        .collect();

        Self { instructions, nodes }
    }

    pub fn count_steps_until<F>(&self, from: &str, until: F) -> u64 where F: Fn(&str) -> bool {
        let mut current = from;
        let mut steps = 0;

        for x in self.instructions.chars().cycle() {
            if until(current) {
                break;
            }

            if let Some(node) = self.nodes.get(current) {
                match x {
                    'L' => current = node.0,
                    'R' => current = node.1,
                    _ => unimplemented!(),
                }
            }

            steps += 1;
        }

        steps
    }

    pub fn part2_nodes(&self) -> impl Iterator<Item = &&str> {
        self.nodes.keys().filter(|key| key.ends_with('A'))
    }
}

#[cfg(test)]
mod answers {
    use crate::prelude::*;
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT_1: &str = include_str!("./input/day8-sample-1.txt");
    const SAMPLE_INPUT_2: &str = include_str!("./input/day8-sample-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("./input/day8-sample-3.txt");
    const PERSONAL_INPUT: &str = include_str!("./input/day8-real.txt");

    #[test_case(SAMPLE_INPUT_1 => 2; "with first sample data")]
    #[test_case(SAMPLE_INPUT_2 => 6; "with second sample data")]
    #[test_case(PERSONAL_INPUT => 24253; "with personal data")]
    pub fn problem1(input: &str) -> u64 {
        Map::parse(input).count_steps_until("AAA", |x| x == "ZZZ")
    }

    #[test_case(SAMPLE_INPUT_3 => 6; "with first sample data")]
    #[test_case(PERSONAL_INPUT => 12357789728873; "with personal data")]
    pub fn problem2(input: &str) -> u64 {
        let map = Map::parse(input);
        map.part2_nodes()
            .map(|&start| { map.count_steps_until(start, |x| x.ends_with('Z')) })
            .reduce(lcm)
            .expect("Well that's just bad math")
    }
}
