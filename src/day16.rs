//! Day 16 - The Floor Will Be Lava

use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Display};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub max_row: usize,
    pub max_col: usize,
}

impl Position {
    pub fn going(&self, dir: Direction) -> Option<(Self, Direction)> {
        match dir {
            Direction::North => (self.row > 0).then_some(Self { row: self.row - 1, ..*self }),
            Direction::South => (self.row < self.max_row).then_some(Self { row: self.row + 1, ..*self }),
            Direction::West => (self.col > 0).then_some(Self { col: self.col - 1, ..*self }),
            Direction::East => (self.col < self.max_col).then_some(Self { col: self.col + 1, ..*self }),
        }
        .map(|p| (p, dir))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

#[derive(Default)]
pub struct Grid {
    nodes: HashMap<Position, char>,
    pub max_row: usize,
    pub max_col: usize,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let max_row = lines.len() - 1;
        let max_col = lines[0].len() - 1;

        let mut this = Self { max_row, max_col, ..Default::default() };

        for (row, line) in lines.into_iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let node = this.position(row, col);
                this.nodes.insert(node, ch);
            }
        }

        this
    }

    pub fn position(&self, row: usize, col: usize) -> Position {
        Position {
            row,
            col,
            max_row: self.max_row,
            max_col: self.max_col,
        }
    }

    pub fn energize(&self, start_pos: Position, direction: Direction) -> HashMap<Position, HashSet<Direction>> {
        let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::new();
        let mut queue = VecDeque::from([(start_pos, direction)]);

        while let Some((next, direction)) = queue.pop_front() {
            let dirs = visited.entry(next.clone()).or_default();
            if !dirs.contains(&direction) {
                dirs.insert(direction.clone());
                queue.extend(self.propagate_beam(next, direction));
            }
        }

        visited
    }

    pub fn propagate_beam(&self, next: Position, direction: Direction) -> impl IntoIterator<Item = (Position, Direction)> {
        match self.nodes.get(&next) {
            Some('.') => vec![ next.going(direction.clone()) ],
            Some('/') => {
                match direction {
                    Direction::North => vec![ next.going(Direction::East) ],
                    Direction::South => vec![ next.going(Direction::West) ],
                    Direction::West => vec![ next.going(Direction::South) ],
                    Direction::East => vec![ next.going(Direction::North) ],
                }
            },
            Some('\\') => {
                match direction {
                    Direction::North => vec![ next.going(Direction::West) ],
                    Direction::South => vec![ next.going(Direction::East) ],
                    Direction::West => vec![ next.going(Direction::North) ],
                    Direction::East => vec![ next.going(Direction::South) ],
                }
            },
            Some('-') => {
                match direction {
                    Direction::West | Direction::East => vec![ next.going(direction.clone()) ],
                    Direction::North | Direction::South => vec![ next.going(Direction::East), next.going(Direction::West) ],
                }
            }
            Some('|') => {
                match direction {
                    Direction::North | Direction::South => vec![ next.going(direction.clone()) ],
                    Direction::East | Direction::West => vec![ next.going(Direction::North), next.going(Direction::South) ],
                }
            }
            _ => unimplemented!("wtf")
        }
        .into_iter().flatten()
    }
}

#[cfg(test)]
mod answers {
    use std::collections::BinaryHeap;
    use super::*;
    use test_case::test_case;

    const SAMPLE: &str = include_str!("./input/day16-sample.txt");
    const PERSONAL: &str = include_str!("./input/day16-real.txt");

    #[test_case(SAMPLE => 46; "with sample data")]
    #[test_case(PERSONAL => 6816; "with personal data")]
    pub fn problem1(input: &str) -> usize {
        let grid = Grid::parse(input);
        let start = grid.position(0, 0);
        grid.energize(start, Direction::East).len()
    }

    #[test_case(SAMPLE => 51; "with sample data")]
    #[test_case(PERSONAL => 8163; "with personal data")]
    pub fn problem2(input: &str) -> usize {
        let grid = Grid::parse(input);
        let origin = grid.position(0, 0);

        let mut bh = BinaryHeap::<usize>::new();

        for col in 0..=origin.max_col {
            let top = grid.position(0, col);
            bh.push(grid.energize(top, Direction::South).len());

            let bottom = grid.position(origin.max_row, col);
            bh.push(grid.energize(bottom, Direction::North).len());
        }

        for row in 0..=origin.max_row {
            let left = grid.position(row, 0);
            bh.push(grid.energize(left, Direction::East).len());

            let right = grid.position(row, origin.max_col);
            bh.push(grid.energize(right, Direction::West).len());
        }

        bh.pop().unwrap()
    }
}
