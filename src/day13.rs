//! Day 13 - Point of Incidence

pub fn parse_image(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines().collect::<Vec<_>>();
    let nrows = lines.len();
    let ncols = lines[0].len();

    let mut rows = vec![0; nrows];
    let mut cols = vec![0; ncols];
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().rev().enumerate() {
            if ch == '#' {
                rows[y] |= 1 << x;
                cols[ncols-x-1] |= 1 << (nrows-y-1);
            }
        }
    }

   (rows, cols)
}

// Find a reflection that contains exactly _tolerance_ errors
fn find_incidence(values: &[u32], tolerance: u32) -> usize {
    for (i, _) in values.windows(2).enumerate() {
        let q = values[..=i].iter().rev();
        let p = values[i+1..].iter();

        if tolerance == q.zip(p).filter(|(&a, &b)| a != b).map(|(a, b)| (a ^ b).count_ones()).sum() {
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
        input.split("\n\n").map(|image| {
            let (rows, cols) = parse_image(image);
            find_incidence(&rows, tolerance) * 100 + find_incidence(&cols, tolerance)
        })
        .sum()
    }
}
