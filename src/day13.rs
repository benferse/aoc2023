//! Day 13 - Point of Incidence

pub struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Pattern {
    pub fn parse(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let nrows = lines.len();
        let ncols = lines[0].len();

        let mut rows = vec![0; nrows];
        let mut cols = vec![0; ncols];

        // <-------- x
        // #.#.#.#.#.# y
        // #.#.#.#.#.# |
        // #.#.#.#.#.# |
        // #.#.#.#.#.# v

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().rev().enumerate() {
                if ch == '#' {
                    rows[y] |= 1 << x;
                    cols[ncols-x-1] |= 1 << (nrows-y-1);
                }
            }
        }

        Self { rows, cols }
    }

    pub fn score(&self, tolerance: u32) -> usize {
        find_incidence(&self.rows, tolerance) * 100 + find_incidence(&self.cols, tolerance)
    }
}

fn find_incidence(values: &[u32], tolerance: u32) -> usize {
    for (i, _) in values.windows(2).enumerate() {
        let backwards = &values[..=i];
        let forwards = &values[i+1..];

        let q = backwards.iter().rev();
        let p = forwards.iter();

        let (is_valid, num_smudges) = q.zip(p).fold((true, 0), |mut state, (a, b)| {
            // If the reflecton is still valid but we noted a difference, allow
            // at most tolerance smudges
            if state.0 && a != b {
                if state.1 < tolerance && (a ^ b) & ((a ^ b) - 1) == 0 {
                    state.1 += 1;
                } else {
                    state.0 = false;
                }
            }
            state
        });

        if is_valid && num_smudges == tolerance {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;
 
    const SAMPLE: &str = include_str!("./input/day13-sample.txt");
    const PERSONAL: &str = include_str!("./input/day13-real.txt");

    #[test_case(SAMPLE, 0 => 405; "with sample data")]
    #[test_case(PERSONAL, 0 => 37113; "with personal data")]
    #[test_case(SAMPLE, 1 => 400; "with sample data and smudges")]
    #[test_case(PERSONAL, 1 => 30449; "with personal data and smudges")]
    pub fn problem_1_and_2(input: &str, tolerance: u32) -> usize {
        input.split("\n\n")
            .map(|p| Pattern::parse(p).score(tolerance))
            .sum()
    }
}
