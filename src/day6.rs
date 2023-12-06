//! Day 6 - Wait For It

type Document<'a> = &'a [(i64, i64)];

pub fn solve_brute_force(races: Document) -> i64 {
    races.iter().map(|(time, distance)| {
        (0..*time).fold(0, |mut accum, x| {
            if x * (time - x) > *distance {
                accum += 1i64;
            }

            accum
        })
    }).product()
}

pub fn solve_quadratic(races: Document) -> i64 {
    races.iter().map(|(time, distance)| {
        let discrim = ((time.pow(2) - 4 * distance) as f64).sqrt();

        // *shakes fist at perfect squares* Am I doing floating point equality
        // comparison here? I sure am. Am I confident it's okay because I know
        // the provenance of the pre-converted integers? Somewhat.
        let mut lower_bound = (*time as f64 - discrim) / 2.0;
        if lower_bound.trunc() == lower_bound {
            // Lower bound was a tie, not a win, so don't count it
            lower_bound += 1.0;
        }

        let mut upper_bound = (*time as f64 + discrim) / 2.0;
        if upper_bound.trunc() == upper_bound {
            // Upper bound was a tie, not a win, so don't count it
            upper_bound -= 1.0;
        }

        (upper_bound.floor() - lower_bound.ceil() + 1.0).trunc() as i64
    })
    .product()
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT_1 => 288; "problem 1 with sample input")]
    #[test_case(PERSONAL_INPUT_1 => 1083852; "problem 1 with real input")]
    #[test_case(SAMPLE_INPUT_2 => 71503; "problem 2 with sample input")]
    #[test_case(PERSONAL_INPUT_2 => 23501589; "problem 2 with real input")]
    pub fn problem_1_and_2_brute_force(input: Document) -> i64 {
        solve_brute_force(input)
    }

    #[test_case(SAMPLE_INPUT_1 => 288; "problem 1 with sample input")]
    #[test_case(PERSONAL_INPUT_1 => 1083852; "problem 1 with real input")]
    #[test_case(SAMPLE_INPUT_2 => 71503; "problem 2 with sample input")]
    #[test_case(PERSONAL_INPUT_2 => 23501589; "problem 2 with real input")]
    pub fn problem_1_and_2_quadratic(input: Document) -> i64 {
        solve_quadratic(input)
    }

    const SAMPLE_INPUT_1: Document = &[ (7, 9,), (15, 40), (30, 200), ];
    const SAMPLE_INPUT_2: Document = &[ (71530, 940200) ];

    const PERSONAL_INPUT_1: Document = &[ (38, 241), (94, 1549), (79, 1074), (70, 1091), ];
    const PERSONAL_INPUT_2: Document = &[ (38_947_970, 241_154_910_741_091), ];
}
