use super::{day::Day, part::Part, part_status::PartStatus, year::Year};

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct ProgressReport {
    year: Year,
    day: Day,
    part: Part,
    status: PartStatus,
}

impl ProgressReport {
    pub fn year(&self) -> Year {
        self.year
    }
    pub fn day(&self) -> Day {
        self.day
    }
    pub fn part(&self) -> Part {
        self.part
    }
    pub fn status(&self) -> PartStatus {
        self.status
    }
}

impl ProgressReport {
    pub fn new<Y: Into<Year>, D: Into<Day>, P: Into<Part>>(
        year: Y,
        day: D,
        part: P,
        status: PartStatus,
    ) -> Self {
        Self {
            year: year.into(),
            day: day.into(),
            part: part.into(),
            status,
        }
    }
}
