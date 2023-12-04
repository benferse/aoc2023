//! Day 4 - Scratchcards

use std::collections::HashSet;

pub struct Card {
    winning_numbers: HashSet<u32>,
    our_numbers: HashSet<u32>,
}

pub fn parse_number_list(input: &str) -> HashSet<u32> {
    input.split_whitespace().flat_map(|s| s.parse()).collect()
}

impl Card {
    pub fn parse(line: &str) -> Self {
        let (_, contents) = line.split_once(':').expect("Should have started with 'Card #:'");
        let (winners, ours) = contents.split_once('|').expect("Should have had two sets of numbers");

        Self {
            winning_numbers: winners.split_whitespace().flat_map(|s| s.parse()).collect(),
            our_numbers: ours.split_whitespace().flat_map(|s| s.parse()).collect(),
        }
    }

    pub fn winners(&self) -> usize {
        self.winning_numbers.intersection(&self.our_numbers).count()
    }

    pub fn points(&self) -> usize {
        if self.winners() > 0 {
            1 << (self.winners() - 1)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod answers {
    use crate::prelude::*;
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 13; "with sample data")]
    #[test_case(PERSONAL_INPUT => 23941; "with real data")]
    pub fn problem1(input: &str) -> usize {
        input.trimmed_lines()
            .map(|line| Card::parse(line).points())
            .sum()
    }

    #[test_case(SAMPLE_INPUT => 30; "with sample data")]
    #[test_case(PERSONAL_INPUT => 5571760; "with personal data")]
    pub fn problem2(input: &str) -> usize {
        input.trimmed_lines()
            .map(|line| Card::parse(line).winners())
            .enumerate()
            .fold(vec![], |mut counts, (idx, winners)| {
                // Need room for num_hits more elements at least
                let new_size = counts.len().max(idx + winners + 1);
                counts.resize(new_size, 1);

                // Record n more of each of the following num_hits cards,
                // where n is the number of copies of the current card we have
                for j in 1..= winners {
                    counts[idx + j] += counts[idx];
                }

                counts
            })
            .iter()
            .sum()
    }

    const SAMPLE_INPUT: &str = 
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const PERSONAL_INPUT: &str = include_str!("./input/day4.txt");
}
