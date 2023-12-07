//! Day 7 - Camel Cards

use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hand {
    r#type: Vec<u8>,
    strength: usize,
    bid: usize,
}

pub fn compare_freq(lhs: &(&char, &u8), rhs: &(&char, &u8)) -> Ordering {
    match rhs.1.cmp(lhs.1) {
        Ordering::Equal => rhs.0.cmp(lhs.0),
        other => other,
    }
}

const RANKS_SANS_JOKERS: &str = "23456789TJQKA";
const RANKS_WITH_JOKERS: &str = "J23456789TQKA";
const JOKER: &char = &'J';

impl Hand {
    pub fn parse(cards: &str, bid: &str, handle_jokers: bool) -> Self {
        let mut frequencies: HashMap<char, u8> = HashMap::new();
        let mut strength = 0;
        let ranks = if handle_jokers { RANKS_WITH_JOKERS } else { RANKS_SANS_JOKERS };

        // Count the type of each card, and interpret the entire hand as a base-13
        // number
        for (idx, card) in cards.chars().rev().take(5).enumerate() {
            *frequencies.entry(card).or_default() += 1;
            strength += (ranks.find(card).unwrap() + 1) * (13_usize.pow(idx as u32));
        }

        if handle_jokers {
            // Make an adjustment here - add the jokers to the item with
            // the highest total so far - e.g. the element with the highest
            // frequency and card value
            if let Some(num_jokers) = frequencies.get(JOKER) {
                let mut x = frequencies.iter().filter(|e| e.0 != JOKER).collect::<Vec<_>>();
                x.sort_unstable_by(compare_freq);

                // If there were non-jokers, adjust the counts of the highest other
                // card and remove the jokers.
                if let Some((&best_card, _)) = x.first() {
                    let num_jokers = *num_jokers;
                    frequencies.entry(best_card).and_modify(|e| *e += num_jokers);
                    frequencies.remove(JOKER);
                }
            }
        }

        // Collect the frequencies into a card classification - e.g.
        // [5] for five-of-a-kind, [4, 1] for four of a kind, etc.
        let mut r#type: Vec<_> = frequencies.into_values().collect();
        r#type.sort_unstable_by(|a, b| b.cmp(a));

        Self {
            r#type,
            strength,
            bid: bid.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod answers {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT, false => 6440; "with sample data")]
    #[test_case(PERSONAL_INPUT, false => 249483956; "with real data")]
    #[test_case(SAMPLE_INPUT, true => 5905; "with sample data and jokers")]
    #[test_case(PERSONAL_INPUT, true => 252137472; "with real data and jokers")]
    pub fn problem_1_and_2(input: &str, handle_jokers: bool) -> usize {
        // Collect all of the hands into a min heap
        let hands = input.lines()
            .filter_map(|line| line.split_once(' '))
            .map(|(hand, bid)| Hand::parse(hand, bid, handle_jokers))
            .map(Reverse)
            .collect::<BinaryHeap<_>>();

        // Pull them off in order and assign winnings
        hands.into_iter_sorted().enumerate().map(|(idx, hand)| hand.0.bid * (idx + 1)).sum()
    }

    const SAMPLE_INPUT: &str = include_str!("./input/day7-sample.txt");
    const PERSONAL_INPUT: &str = include_str!("./input/day7-real.txt");
}
