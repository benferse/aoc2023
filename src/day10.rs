//! Day 10 - Pipe Maze

use std::collections::{HashSet, VecDeque};

pub fn load_map(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn locate_start(map: &mut [Vec<u8>]) -> (usize, usize) {
    for (row_num, row) in map.iter_mut().enumerate() {
        for (col_num, node) in row.iter_mut().enumerate() {
            if *node == b'S' {
                *node = b'F';
                return (row_num, col_num)
            }
        }
    }
    unimplemented!("There was no starting node");
}

pub fn edges_of(map: &[Vec<u8>], location: &(usize, usize)) -> [(usize, usize); 2] {
    match map[location.0][location.1] {
        b'|' => [(location.0 - 1, location.1),     (location.0 + 1, location.1)],
        b'-' => [(location.0,     location.1 - 1), (location.0,     location.1 + 1)],
        b'L' => [(location.0 - 1, location.1),     (location.0,     location.1 + 1)],
        b'J' => [(location.0 - 1, location.1),     (location.0,     location.1 - 1)],
        b'7' => [(location.0 + 1, location.1),     (location.0,     location.1 - 1)],
        b'F' => [(location.0 + 1, location.1),     (location.0,     location.1 + 1)],
        unknown  => unimplemented!("What on earth is this {unknown} doing in the loop"),
    }
}

pub fn extract_ring(map: &[Vec<u8>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::from([start]);
    let mut q = VecDeque::from([edges_of(map, &start)[0]]);

    while let Some(next) = q.pop_back() {
        for edge in edges_of(map, &next) {
            if visited.insert(edge) {
                q.push_front(edge);
            }
        }
    }

    visited
}

pub fn calculate_inner_area(map: &[Vec<u8>], ring: &HashSet<(usize, usize)>) -> usize {
    let mut area = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            // By definition, nodes on the loop are not enclosed
            if ring.contains(&(y, x)) {
                continue;
            }

            // Ray casting for the win
            let count = (x..row.len()).zip(y..map.len()).filter(|&(x2, y2)| {
                let c = map[y2][x2];
                ring.contains(&(y2, x2)) && c != b'L' && c != b'7'
            }).count();

            if count % 2 == 1 {
                area += 1;
            }
        }
    }

    area
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
        let mut map = load_map(input);
        let start = locate_start(&mut map);
        let ring = extract_ring(&map, start);

        ring.len() / 2
    }

    #[test_case(SAMPLE_INPUT_3 => 4; "with third sample data")]
    #[test_case(PERSONAL_INPUT => 357; "with personal data")]
    pub fn problem2(input: &str) -> usize {
        let mut map = load_map(input);
        let start = locate_start(&mut map);
        let ring = extract_ring(&map, start);

        calculate_inner_area(&map, &ring)
    }
}
