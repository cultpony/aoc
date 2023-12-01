use std::str::FromStr;

use super::part::Part;

use super::day::Day;

use super::year::Year;
use super::Puzzle;

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct PuzzleSpec {
    pub(crate) year: Option<Year>,
    pub(crate) day: Option<Day>,
    pub(crate) part: Option<Part>,
}

impl FromStr for PuzzleSpec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (year, day, part): (Option<&str>, Option<&str>, Option<&str>) = s
            .split_once('.')
            .or(Some((s, "")))
            .map(|(pass, rest)| {
                let (day, part) = rest
                    .split_once('.')
                    .or(Some((rest, "")))
                    .map(|(a, b)| (Some(a), Some(b)))
                    .unwrap_or((None, None));
                (Some(pass), day, part)
            })
            .map(|(year, day, part)| {
                let year = year.filter(|x| *x != "*" && *x != "-" && !x.is_empty());
                let day = day.filter(|x| *x != "*" && *x != "-" && !x.is_empty());
                let part = part.filter(|x| *x != "*" && *x != "-" && !x.is_empty());
                (year, day, part)
            })
            .unwrap_or((None, None, None));
        let year: Option<u16> = year.map(|x| x.parse()).transpose()?;
        let day: Option<u8> = day.map(|x| x.parse()).transpose()?;
        let part: Option<u8> = part.map(|x| x.parse()).transpose()?;
        Ok(Self {
            year: year.map(Into::into),
            day: day.map(Into::into),
            part: part.map(Into::into),
        })
    }
}

impl PartialEq<Puzzle> for PuzzleSpec {
    fn eq(&self, other: &Puzzle) -> bool {
        let year_eq = self.year.map(|year| year == other.year()).unwrap_or(true);
        let day_eq = self.day.map(|day| day == other.day()).unwrap_or(true);
        let part_eq = self.part.map(|part| part == other.part()).unwrap_or(true);
        year_eq && day_eq && part_eq
    }
}
