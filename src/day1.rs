//! Day 1 - Trebuchet?!

/// Given a line of text from the document, extract the
/// correct calibration value
///
/// ```
/// # use aoc2023::day1::*;
/// assert_eq!(calibration_v1("1abc2"), 12);
/// assert_eq!(calibration_v1("pqr3stu8vwx"), 38);
/// assert_eq!(calibration_v1("a1b2c3d4e5f"), 15);
/// assert_eq!(calibration_v1("treb7uchet"), 77);
/// ```
pub fn calibration_v1(line: &str) -> u32 {
    let first = line.chars()
        .find_map(|c| c.to_digit(10))
        .expect("Should have had a first digit");

    let last = line.chars().rev()
        .find_map(|c| c.to_digit(10))
        .expect("Should have had a last digit");

    first * 10 + last
}

/// Given a line of text from the document, find the
/// calibration value, assuming english digit words are
/// their corresponding digits
///
/// ```
/// # use aoc2023::day1::*;
/// assert_eq!(calibration_v2("two1nine"), 29);
/// assert_eq!(calibration_v2("eightwo"), 82);
/// ```
pub fn calibration_v2(line: &str) -> u32 {
    let first = (0..line.len())
        .find_map(|idx| is_digit_or_number_word(&line[idx..]))
        .expect("Should have had a first digit");

    let last = (0..line.len()).rev()
        .find_map(|idx| is_digit_or_number_word(&line[idx..]))
        .expect("Should have had a last digit");

    first * 10 + last
}

/// Given a string slice, see if it starts with either a digit word or
/// an ascii digit
fn is_digit_or_number_word(text: &str) -> Option<u32> {
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
        .enumerate()
        .find(|(_, &number_word)| text.starts_with(number_word))
        .map(|(index, _)| index as u32 + 1)
        .or_else(|| text.chars().next().expect("Ran out of characters").to_digit(10))
}

#[cfg(test)]
mod answers {
    use crate::prelude::*;
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT_1 => 142; "with sample input")]
    #[test_case(PERSONAL_INPUT => 55090; "with real input")]
    pub fn problem1(input: &str) -> u32 {
        input.trimmed_lines()
            .map(calibration_v1)
            .sum()
    }

    #[test_case(SAMPLE_INPUT_2 => 281; "with sample input")]
    #[test_case(PERSONAL_INPUT => 54845; "with real input")]
    pub fn problem2(input: &str) -> u32 {
        input.trimmed_lines()
            .map(calibration_v2)
            .sum()
    }

    const SAMPLE_INPUT_1: &str =
       "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    const SAMPLE_INPUT_2: &str =
       "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    const PERSONAL_INPUT: &str = include_str!("./input/day1.txt");
}
