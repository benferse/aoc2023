//! Day 15 - Lens Library

/// Calculate the Holiday ASCII String Helper value
/// of a given string
///
/// Examples
/// ```
/// # use aoc2023::day15::holiday_hash;
///
/// assert_eq!(52, holiday_hash("HASH"));
/// ```
pub fn holiday_hash(input: &str) -> usize {
    input.as_bytes().iter().fold(0u8, |accum, &x| {
        accum.wrapping_add(x).wrapping_mul(17)
    }) as usize
}

/// Parse a single step
///
/// Examples
/// ```
/// # use aoc2023::day15::parse_step;
///
/// assert_eq!(parse_step("ot=7"), ("ot", '=', Some(7)));
/// assert_eq!(parse_step("cn-"), ("cn", '-', None));
/// ```
pub fn parse_step(step: &str) -> (&str, char, Option<usize>) {
    let op_idx = step.find(|ch| ch == '=' || ch == '-').unwrap();
    let (label, rest) = step.split_at(op_idx);
    (label, rest.chars().next().unwrap(), rest[1..].parse().ok())
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    const SAMPLE: &str = include_str!("./input/day15-sample.txt");
    const PERSONAL: &str = include_str!("./input/day15-real.txt");

    #[test_case(SAMPLE => 1320; "with sample data")]
    #[test_case(PERSONAL => 517965; "with personal data")]
    pub fn problem1(input: &str) -> usize {
        input.trim_end().split(',').fold(0, |x, y| x + holiday_hash(y))
    }

    #[test_case(SAMPLE => 145; "with sample data")]
    #[test_case(PERSONAL => 267372; "with personal data")]
    pub fn problem2(input: &str) -> usize {
        let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

        for (label, op, arg) in input.trim_end().split(',').map(parse_step) {
            let this_box = &mut boxes[holiday_hash(label)];
            match (op, arg) {
                ('-', _) => {
                    this_box.retain(|b| b.0 != label);
                },
                ('=', Some(flen)) => {
                    match this_box.iter_mut().find(|e| e.0 == label) {
                        Some(spot) => spot.1 = flen,
                        None => this_box.push((label, flen)),
                    }
                },
                _ => unimplemented!("wtf")
            };
        }

        boxes.into_iter().enumerate().flat_map(|(bn, b)| 
            b.into_iter().enumerate().map(move |(sn, s)| (bn + 1) * (sn + 1) * s.1)
        )
        .sum()
    }
}
