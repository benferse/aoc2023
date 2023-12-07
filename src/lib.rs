//! Advent of Code - 2023

#![feature(binary_heap_into_iter_sorted)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(lazy_cell)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub mod prelude {
    pub struct TrimmedLines<'a>(std::str::Lines<'a>);

    impl<'a> Iterator for TrimmedLines<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(str::trim)
        }
    }

    pub trait HasTrimmedLines {
        fn trimmed_lines(&self) -> TrimmedLines;
    }

    impl HasTrimmedLines for &str {
        fn trimmed_lines(&self) -> TrimmedLines {
            TrimmedLines(self.lines())
        }
    }
}
