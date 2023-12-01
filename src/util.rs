pub mod parsing;

use std::{
    io::{Cursor, Seek, Write},
    sync::Arc,
};

use aoc_client::AocClient;

use crate::aoc::{
    day::Day, get_puzzle, part::Part, part_status::PartStatus, progress_report::ProgressReport,
    puzzlespec::PuzzleSpec, year::Year,
};
use anyhow::{Context, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionOutcome {
    Correct,
    Incorrect,
    Wait,
    WrongLevel,
}

impl From<aoc_client::SubmissionOutcome> for SubmissionOutcome {
    fn from(value: aoc_client::SubmissionOutcome) -> Self {
        match value {
            aoc_client::SubmissionOutcome::Correct => Self::Correct,
            aoc_client::SubmissionOutcome::Incorrect => Self::Incorrect,
            aoc_client::SubmissionOutcome::Wait => Self::Wait,
            aoc_client::SubmissionOutcome::WrongLevel => Self::WrongLevel,
        }
    }
}

#[repr(transparent)]
pub struct SecretString(Arc<String>);

impl std::str::FromStr for SecretString {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Arc::new(s.to_owned())))
    }
}

impl SecretString {
    pub fn unpack(self) -> String {
        (*self.0).clone()
    }
}

impl Clone for SecretString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(clap::Parser)]
pub struct Arguments {
    #[clap(env, long, short, hide_env = true)]
    aoc_token: SecretString,
    pub action: Action,
    #[clap(value_parser = clap::value_parser!(PuzzleSpec))]
    pub puzzle: Option<PuzzleSpec>,
    #[clap(env, long, short, default_value = "1000")]
    pub bench_loops: u32,
}

#[derive(Default, Debug, clap::ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    #[default]
    #[clap(name = "list-aoc", alias = "list")]
    ListAoC,
    #[clap(name = "run")]
    Run,
    #[clap(name = "bench", alias = "benchmark")]
    Benchmark,
}

impl Arguments {
    fn get_client<Y: Into<Year>, D: Into<Day>>(&self, year: Y, day: D) -> Result<AocClient, Error> {
        let year: Year = year.into();
        let day: Day = day.into();
        Ok(aoc_client::AocClient::builder()
            .session_cookie(self.aoc_token.clone().unpack())?
            .year(year.as_u16().into())?
            .day(day.as_u8().into())?
            .build()?)
    }

    pub fn bench_loops(&self) -> u32 {
        self.bench_loops
    }

    pub fn submit_solution<
        Y: Into<Year> + Copy,
        D: Into<Day> + Copy,
        P: Into<Part>,
        S: Into<String>,
    >(
        &self,
        year: Y,
        day: D,
        part: P,
        data: S,
    ) -> Result<SubmissionOutcome, Error> {
        let data: String = data.into();
        let part: Part = part.into();
        let client = self.get_client(year, day)?;
        let result: SubmissionOutcome = client.submit_answer(part.as_i64(), data)?.into();

        let mut cache = scratch::path("aoc_req");
        let year: Year = year.into();
        let day: Day = day.into();
        cache.push(format!("year{year}_day{day}_input"));

        if cache.exists() {
            std::fs::remove_file(cache)?;
        }

        let mut cache = scratch::path("aoc_req");
        cache.push(format!("year{year}_calendar"));

        if cache.exists() {
            std::fs::remove_file(cache)?;
        }

        Ok(result)
    }

    pub fn get_input<Y: Into<Year>, D: Into<Day>>(&self, year: Y, day: D) -> Result<String, Error> {
        let mut cache = scratch::path("aoc_req");
        let year: Year = year.into();
        let day: Day = day.into();
        cache.push(format!("year{year}_day{day}_input"));

        if let Ok(file) = std::fs::File::open(cache.clone()) {
            Ok(std::io::read_to_string(file)?)
        } else {
            let data = self.get_client(year, day)?.get_input()?;
            let mut cache = std::fs::File::options()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(cache.clone())
                .context("could not create cache file")?;

            cache
                .write_all(data.as_bytes())
                .context("writing cache data")?;

            cache.flush().context("flushing write to disk")?;

            cache
                .seek(std::io::SeekFrom::Start(0))
                .context("seeking back to start")?;

            Ok(std::io::read_to_string(cache)?)
        }
    }

    pub fn get_progress<Y: Into<Year>>(&self, year: Y) -> Result<Vec<ProgressReport>, Error> {
        let year: Year = year.into();
        let mut cache = scratch::path("aoc_req");
        cache.push(format!("year{year}_calendar"));
        let file = if let Ok(file) = std::fs::File::open(cache.clone()) {
            file
        } else {
            let cal = self.get_client(year, 1)?;
            let cal = cal.get_calendar_html()?;
            let cal = Cursor::new(cal);
            let cal = html2text::from_read_with_decorator(
                cal,
                80,
                html2text::render::text_renderer::TrivialDecorator::new(),
            );
            let regex = regex::Regex::new(r#"(?P<day>\d{1,2})\s(?P<p1>\*(?P<p2>\*|$)|$)"#).unwrap();
            let mut data = Vec::new();
            for line in cal.lines() {
                if let Some(captures) = regex.captures(line) {
                    let day: u8 = captures.name("day").unwrap().as_str().parse()?;
                    let first_part = captures.name("p1").map(|x| x.as_str()).unwrap_or_default();
                    let _second_part = captures.name("p2").map(|x| x.as_str()).unwrap_or_default();
                    data.push(ProgressReport::new(
                        year,
                        day,
                        Part::one(),
                        if first_part == "*" || first_part == "**" {
                            PartStatus::Completed
                        } else if get_puzzle(year, day, Part::one()).is_some() {
                            PartStatus::MissingWithSolution
                        } else {
                            PartStatus::MissingWithNoSolution
                        },
                    ));
                    data.push(ProgressReport::new(
                        year,
                        day,
                        Part::two(),
                        if first_part == "**" {
                            PartStatus::Completed
                        } else if get_puzzle(year, day, Part::two()).is_some() {
                            PartStatus::MissingWithSolution
                        } else {
                            PartStatus::MissingWithNoSolution
                        },
                    ));
                }
            }
            let data = serde_json::to_string_pretty(&data)
                .context("could not serialize progress data to cache")?;

            let mut cache = std::fs::File::options()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(cache.clone())
                .context("could not create cache file")?;

            cache
                .write_all(data.as_bytes())
                .context("writing cache data")?;

            cache.flush().context("flushing write to disk")?;

            cache
                .seek(std::io::SeekFrom::Start(0))
                .context("seeking back to start")?;

            cache
        };
        let file: Vec<ProgressReport> =
            serde_json::from_reader(file).context("could not deserialize from reader")?;
        Ok(file)
    }

    pub fn get_day_progress<
        Y: Into<Year> + PartialEq<Year> + Clone + Copy,
        D: Into<Day> + PartialEq<Day> + Clone + Copy,
        P: Into<Part> + PartialEq<Part> + Clone + Copy,
    >(
        &self,
        year: Y,
        day: D,
        part: P,
    ) -> Result<ProgressReport, Error> {
        let progresses = self.get_progress(year)?;
        Ok(progresses
            .into_iter()
            .find(|x| year == x.year() && day == x.day() && part == x.part())
            .unwrap_or_else(|| ProgressReport::new(year, day, part, PartStatus::Unavailable)))
    }
}
