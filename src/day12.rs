//! Day 12 - Hot Springs

use crate::prelude::strings::*;
use std::iter::once;

#[derive(Debug)]
pub struct Record {
    spring_layout: Vec<char>,
    group_sizes: Vec<usize>,
}

impl Record {
    pub fn parse(input: &str, num_folds: usize) -> Self {
        let (x, y) = input.split_once(' ').expect("Bad input");

        // Pre-process the spring layout so we don't have to do it later. Trailing
        // empty spaces can be ignored, and add one at the beginning to make the
        // off-by-ones during counting less awful lol
        let x = once(x).cycle().take(num_folds).join("?");
        let spring_layout = format!(".{}", x.trim_end_matches('.')).chars().collect();

        // For the group sizes, just split and duplicate them
        let group_sizes = y.split(',').filter_map(|c| c.parse().ok()).collect::<Vec<_>>();
        let num_groups = group_sizes.len();
        let group_sizes = group_sizes.into_iter().cycle().take(num_groups * num_folds).collect();

        Self { spring_layout, group_sizes }
    }

    // I really fucking hate dynamic programming
    pub fn count_arrangements(&self) -> usize {
        // The overall approach is to iteratively determine how many
        // configurations there are for the first 0..n groupings in
        // the first 0..m positions. When all is said and done, the last
        // cell in the table should be the number of ways for all of the
        // groupings to appear in all of the positions

        // The first row of the table is the number of ways we can have
        // zero springs in the first n positions. There's exactly one way
        // to have nothing until we hit the first known spring, at which point
        // there are zero ways
        //
        // The zeroth position is trivially 1 - there's one way to put nothing
        // into nothing :)
        let mut previous_table = vec![0; self.spring_layout.len() + 1];
        previous_table[0] = 1;
        for (i, _) in self.spring_layout.iter().take_while(|&&ch| ch != '#').enumerate() {
            previous_table[i + 1] = 1;
        }

        for &group_size in &self.group_sizes {
            // Setup a new table to track the number of ways to position
            // the current group taking into account all of the previous
            // groups
            let mut current_table = vec![0; self.spring_layout.len() + 1];
            let mut non_empty_spots = 0;

            for (position, &ch) in self.spring_layout.iter().enumerate() {
                // Did we just leave the last contiguous block of
                // springs or potential springs?
                if ch != '.' {
                    non_empty_spots += 1;
                } else {
                    non_empty_spots = 0;
                }

                // If the current spot isn't known to be a spring, then we
                // know we can fit at least as many permutations as we could as
                // of the last position for the current grouping. In other words,
                // if it isn't certain that we just moved onto a spring, we can do
                // at least as good as we could in the last position.
                if ch != '#' {
                    current_table[position + 1] += current_table[position];
                }

                // If we found a section that can fit the current group and it's *not*
                // an extension of the previous contiguous block, then the previous
                // m-1 groupings can all be handled by regions preceding this one,
                // so add those in as well
                if non_empty_spots >= group_size && self.spring_layout[position - group_size] != '#' {
                    current_table[position + 1] += previous_table[position - group_size];
                }
            }

            previous_table = current_table;
        }

        // The final entry in the final row of the table is our
        // answer
        previous_table[self.spring_layout.len()]
    }
}

#[cfg(test)]
mod answers {
    use crate::prelude::strings::*;
    use super::*;
    use test_case::test_case;

    const SAMPLE: &str = include_str!("./input/day12-sample.txt");
    const PERSONAL: &str = include_str!("./input/day12-real.txt");

    #[test_case(SAMPLE, 1 => 21; "with sample data")]
    #[test_case(PERSONAL, 1 => 7771; "with personal data")]
    #[test_case(SAMPLE, 5 => 525152; "with sample data and 5 folds")]
    #[test_case(PERSONAL, 5 => 10_861_030_975_833; "with personal data and 5 folds")]
    pub fn problem_1_and_2(input: &str, num_folds: usize) -> usize {
        input.map_lines(|line| {
            let record = Record::parse(line, num_folds);
            record.count_arrangements()
        })
        .sum()
    }
}
