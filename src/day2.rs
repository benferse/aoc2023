//! Day 2 - Cube Conundrum

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Bag {
    pub max_red: usize,
    pub max_green: usize,
    pub max_blue: usize,
}

impl Bag {
    /// Given an input line, build the minimum viable bag for this
    /// game of show and tell
    ///
    /// ```
    /// # use aoc2023::day2::*;
    /// let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    /// let bag = Bag::parse(input);
    /// assert_eq!(bag, Bag { max_red: 4, max_blue: 6, max_green: 2 });
    /// ```
    pub fn parse(input: &str) -> Self {
        let mut result = Self::default();

        if let Some((_, line)) = input.split_once(':') {
            line.split(';')
                .flat_map(|set| set.split(','))
                .map(|shown| shown.trim().split_once(' ').unwrap())
                .for_each(|(num, color)| {
                    let num = num.parse().expect("Should have been a number");
                    match color {
                        "red" => result.max_red = result.max_red.max(num),
                        "blue" => result.max_blue = result.max_blue.max(num),
                        "green" => result.max_green = result.max_green.max(num),
                        err => unimplemented!("Can't handle color {err}"),
                    }
                });
        }

        result
    }
}

#[cfg(test)]
mod answers {
    use crate::prelude::strings::*;
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 8; "with sample data")]
    #[test_case(PERSONAL_INPUT => 2156; "with real data")]
    pub fn problem1(input: &str) -> usize {
        input.map_lines(Bag::parse)
            .enumerate()
            .filter(|(_, bag)| bag.max_red <= 12 && bag.max_green <= 13 && bag.max_blue <= 14)
            .map(|(idx, _)| idx + 1)
            .sum()
    }

    #[test_case(SAMPLE_INPUT => 2286; "with sample data")]
    #[test_case(PERSONAL_INPUT => 66909; "with real data")]
    pub fn problem2(input: &str) -> usize {
        input.map_lines(Bag::parse)
            .map(|bag| bag.max_red * bag.max_blue * bag.max_green)
            .sum()
    }

    const SAMPLE_INPUT: &str = 
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const PERSONAL_INPUT: &str = include_str!("./input/day2.txt");
}
