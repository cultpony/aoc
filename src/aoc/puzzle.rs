use std::panic::RefUnwindSafe;

use crate::util::parsing::Solution;

use super::{day::Day, part::Part, year::Year};

pub const fn puzzle<S: Solution>(
    year: Year,
    day: Day,
    part: Part,
    add: &'static S::AdditionalArguments,
    call: Caller,
) -> Puzzle {
    Puzzle {
        year,
        day,
        part,
        add: (add as *const S::AdditionalArguments) as *const u8,
        call,
    }
}

pub type Caller =
    &'static (dyn Fn(&str, *const u8) -> anyhow::Result<String> + Send + Sync + RefUnwindSafe);

#[derive(Clone)]
pub struct Puzzle {
    year: Year,
    day: Day,
    part: Part,
    add: *const u8,
    call: Caller,
}

unsafe impl Sync for Puzzle {}
unsafe impl Send for Puzzle {}

impl std::fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Puzzle")
            .field("year", &self.year)
            .field("day", &self.day)
            .field("part", &self.part)
            .finish()
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}.{}.{}",
            self.year(),
            self.day(),
            self.part()
        ))
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.day == other.day && self.part == other.part
    }
}

impl Eq for Puzzle {}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.day.cmp(&other.day) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.part.cmp(&other.part)
    }
}

impl Puzzle {
    pub fn year(&self) -> Year {
        self.year
    }
    pub fn day(&self) -> Day {
        self.day
    }
    pub fn part(&self) -> Part {
        self.part
    }
    pub fn call(&self, inp: &str, add: Option<*const u8>) -> anyhow::Result<String> {
        (self.call)(
            inp,
            match add {
                Some(add) => add,
                None => self.add,
            },
        )
    }
}
