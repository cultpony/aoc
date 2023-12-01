use crate::util::parsing::Solution;

use super::{day::Day, part::Part, puzzle::Puzzle, year::Year};

pub const fn puzzle_test<S: Solution>(
    year: Year,
    day: Day,
    part: Part,
    input: &'static str,
    add: &'static S::AdditionalArguments,
    output: &'static str,
) -> PuzzleTestInput {
    PuzzleTestInput {
        year,
        day,
        part,
        input,
        add: (add as *const S::AdditionalArguments) as *const u8,
        output,
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PuzzleTestInput {
    year: Year,
    day: Day,
    part: Part,
    input: &'static str,
    add: *const u8,
    output: &'static str,
}

impl PuzzleTestInput {
    pub fn input(&self) -> &'static str {
        self.input
    }
    pub fn output(&self) -> &'static str {
        self.output
    }
    pub fn add(&self) -> *const u8 {
        self.add
    }
}

unsafe impl Sync for PuzzleTestInput {}
unsafe impl Send for PuzzleTestInput {}

impl PartialOrd for PuzzleTestInput {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PuzzleTestInput {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.day.cmp(&other.day) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.part.cmp(&other.part) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.input.cmp(other.input) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.output.cmp(other.output)
    }
}

impl PartialEq<Puzzle> for PuzzleTestInput {
    fn eq(&self, other: &Puzzle) -> bool {
        self.year == other.year() && self.day == other.day() && self.part == other.part()
    }
}
