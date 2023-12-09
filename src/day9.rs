//! Day 9 - Mirage Maintenance

pub fn parse(input: &str) -> Vec<i64> {
    input.split_whitespace().flat_map(|word| word.parse()).collect()
}

pub fn extrapolate(source: Vec<i64>) -> Vec<Vec<i64>> {
    let mut results = vec![source];

    loop {
        let current = &results[results.len() - 1];
        let next = current.iter().map_windows(|&[a, b]| b - a).collect::<Vec<_>>();
        if next.iter().all(|x| x == &0) {
            break;
        }
        results.push(next);
    }

    results
}

#[cfg(test)]
mod answers {
    use crate::prelude::strings::*;
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = include_str!("./input/day9-sample.txt");
    const PERSONAL_INPUT: &str = include_str!("./input/day9-real.txt");

    #[test_case(SAMPLE_INPUT, false => 114; "with sample input forward")]
    #[test_case(SAMPLE_INPUT, true => 2; "with sample input reversed")]
    #[test_case(PERSONAL_INPUT, false => 1974232246; "with personal input forward")]
    #[test_case(PERSONAL_INPUT, true => 928; "with personal input reversed")]
    pub fn problem1(input: &str, reverse: bool) -> i64 {
        input.map_lines(|line| {
            let mut seq = parse(line);
            if reverse { seq.reverse(); }
            extrapolate(seq).iter().map(|v| v[v.len() - 1]).sum::<i64>()
        })
        .sum()
    }
}
