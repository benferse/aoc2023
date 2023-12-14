//! Day 14 - Parabolic Reflector Dish

use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Platform {
    data: Vec<Vec<char>>,
}

impl Platform {
    pub fn parse(input: &str) -> Self {
        Self { data: input.lines().map(|line| line.chars().collect()).collect() }
    }

    pub fn as_empty(&self) -> Self {
        Self { data: vec![vec!['.'; self.data.len()]; self.data.len()] }
    }

    pub fn rotate_right(&self) -> Self {
        let mut next = self.as_empty();
        let dim = self.data.len();

        for col in 0..dim {
            for row in 0..dim {
                next.data[col][dim-row-1] = self.data[row][col];
            }
        }

        next
    }

    pub fn tilt_north(&self) -> Self {
        let mut next = self.as_empty();
        let dim = self.data.len();

        for col in 0..dim {
            for row in 0..dim {
                match self.data[row][col] {
                    '.' => { 
                        // Just skip these
                    },
                    '#' => {
                        next.data[row][col] = '#';
                    },
                    'O' => {
                        let mut new_row = row;
                        for check in (0..row).rev() {
                            if next.data[check][col] != '.' {
                                break;
                            }

                            new_row -= 1;
                        }

                        next.data[new_row][col] = 'O';
                    }
                    _ => {}
                }
            }
        }

        next
    }

    pub fn calc_weight(&self) -> usize {
        self.data.iter().rev().enumerate().map(|(idx, row)| {
            let num_stones = row.iter().filter(|&ch| ch == &'O').count();
            num_stones * (idx + 1)
        })
        .sum()
    }
}

impl Display for Platform {
    #[allow(unused)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for ch in row {
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    const SAMPLE: &str = include_str!("./input/day14-sample.txt");
    const PERSONAL: &str = include_str!("./input/day14-real.txt");

    #[test_case(SAMPLE => 136; "with sample data")]
    #[test_case(PERSONAL => 109385; "with personal data")]
    pub fn problem1(input: &str) -> usize {
        Platform::parse(input).tilt_north().calc_weight()
    }

    fn do_cycle(platform: Platform) -> Platform {
        platform.tilt_north()
                .rotate_right()
                .tilt_north()
                .rotate_right()
                .tilt_north()
                .rotate_right()
                .tilt_north()
                .rotate_right()
    }

    #[test_case(SAMPLE => 64; "with sample data")]
    pub fn problem2(input: &str) -> usize {
        0
    }
}
