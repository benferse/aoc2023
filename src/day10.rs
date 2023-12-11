//! Day 10 - Pipe Maze

use std::collections::{HashSet, VecDeque};

pub struct Map {
    nodes: Vec<Vec<u8>>,
    start: (usize, usize),
}

impl Map {
    pub fn load(input: &str) -> Self {
        let mut nodes: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        for (y, row) in nodes.iter_mut().enumerate() {
            for (x, node) in row.iter_mut().enumerate() {
                if *node == b'S' {
                    *node = b'F';
                    return Self { nodes, start: (x, y) }
                }
            }
        }

        unimplemented!("There was no starting node");
    }

    pub fn extract_ring(&self) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::from([self.start]);
        let mut q = VecDeque::from([self.edges_of(&self.start)[0]]);

        while let Some(next) = q.pop_back() {
            for edge in self.edges_of(&next) {
                if visited.insert(edge) {
                    q.push_front(edge);
                }
            }
        }

        visited
    }

    pub fn calculate_inner_area(&self) -> usize {
        let ring = self.extract_ring();
        let mut area = 0;

        for (y, row) in self.nodes.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                // By definition, nodes on the loop are not enclosed
                if ring.contains(&(y, x)) {
                    continue;
                }

                // Ray casting for the win
                let count = (x..row.len()).zip(y..self.nodes.len()).filter(|&(x2, y2)| {
                    let c = self.nodes[y2][x2];
                    ring.contains(&(y2, x2)) && c != b'L' && c != b'7'
                }).count();

                if count % 2 == 1 {
                    area += 1;
                }
            }
        }

        area
    }


    fn edges_of(&self, &(x, y): &(usize, usize)) -> [(usize, usize); 2] {
        match self.nodes[x][y] {
            b'|' => [(x - 1, y),     (x + 1, y)],
            b'-' => [(x,     y - 1), (x,     y + 1)],
            b'L' => [(x - 1, y),     (x,     y + 1)],
            b'J' => [(x - 1, y),     (x,     y - 1)],
            b'7' => [(x + 1, y),     (x,     y - 1)],
            b'F' => [(x + 1, y),     (x,     y + 1)],
            unknown  => unimplemented!("What on earth is this {unknown} doing in the loop"),
        }
    }
}


#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT_1: &str = include_str!("./input/day10-sample-1.txt");
    const SAMPLE_INPUT_2: &str = include_str!("./input/day10-sample-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("./input/day10-sample-3.txt");
    const PERSONAL_INPUT: &str = include_str!("./input/day10-real.txt");

    #[test_case(SAMPLE_INPUT_1 => 4; "with first sample data")]
    #[test_case(SAMPLE_INPUT_2 => 8; "with second sample data")]
    #[test_case(PERSONAL_INPUT => 6931; "with personal data")]
    pub fn problem1(input: &str) -> usize {
        Map::load(input).extract_ring().len() / 2
    }

    #[test_case(SAMPLE_INPUT_3 => 4; "with third sample data")]
    #[test_case(PERSONAL_INPUT => 357; "with personal data")]
    pub fn problem2(input: &str) -> usize {
        Map::load(input).calculate_inner_area()
    }
}
