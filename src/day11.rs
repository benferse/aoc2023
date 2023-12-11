//! Day 11 - Cosmic Expansion

pub type Universe = Vec<(usize, usize)>;

pub fn parse_image(input: &str) -> Universe {
    let mut universe = Universe::new();

    for (row, line) in input.lines().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            if cell == '#' {
                universe.push((row, col));
            }
        }
    }

    universe
}

pub fn expansion(universe: &mut Universe, factor: usize) {
    let mut row_extents = (usize::MAX, usize::MIN);
    let mut col_extents = (usize::MAX, usize::MIN);

    universe.iter().for_each(|loc| {
        row_extents = (row_extents.0.min(loc.0), row_extents.1.max(loc.0));
        col_extents = (col_extents.0.min(loc.1), col_extents.1.max(loc.1));
    });

    for row in (row_extents.0..row_extents.1).rev() {
        // If this row is empty, every row below it should be adjusted by
        // factor
        if !universe.iter().any(|x| x.0 == row) {
            universe.iter_mut().filter(|j| j.0 > row).for_each(|k| k.0 += factor);
        }
    }

    for col in (col_extents.0..col_extents.1).rev() {
        if !universe.iter().any(|x| x.1 == col) {
            universe.iter_mut().filter(|j| j.1 > col).for_each(|k| k.1 += factor);
        }
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = include_str!("./input/day11-sample.txt");
    const PERSONAL_INPUT: &str = include_str!("./input/day11-real.txt");

    #[test_case(SAMPLE_INPUT, 1 => 374; "with sample data")]
    #[test_case(PERSONAL_INPUT, 1 => 9_742_154; "with personal data")]
    #[test_case(SAMPLE_INPUT, 9 => 1030; "with sample data x10")]
    #[test_case(SAMPLE_INPUT, 99 => 8410; "with sample data x100")]
    #[test_case(PERSONAL_INPUT, 999_999 => 411_142_919_886; "with personal data x1000000")]
    pub fn problem1(input: &str, factor: usize) -> usize {
        let mut universe = parse_image(input);
        expansion(&mut universe, factor);

        let mut distance = 0;

        for (n, x) in universe.iter().enumerate() {
            for y in universe.iter().skip(n + 1) {
                distance += x.0.abs_diff(y.0) + x.1.abs_diff(y.1);
            }
        }

        distance
    }
}
