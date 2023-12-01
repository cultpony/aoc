use std::{convert::Infallible, str::FromStr};

use crate::aoc::day::day;
use crate::aoc::year::year;
use crate::{
    add_solution, add_test,
    aoc::{day::Day, year::Year},
    util::parsing::{Lines, PlainUnpacker, Solution},
};
use anyhow::Result;

add_solution!(S);
add_test!(S, part 1, "1abc2" => "12");
add_test!(S, part 1, "pqr3stu8vwx" => "38");
add_test!(S, part 1, "a1b2c3d4e5f" => "15");
add_test!(S, part 1, "treb7uchet" => "77");
add_test!(S, part 2, "two1nine" => "29");
add_test!(S, part 2, "eightwothree" => "83");
add_test!(S, part 2, "abcone2threexyz" => "13");
add_test!(S, part 2, "xtwone3four" => "24");
add_test!(S, part 2, "4nineeightseven2" => "42");
add_test!(S, part 2, "zoneight234" => "14");
add_test!(S, part 2, "7pqrstsixteen" => "76");

pub struct S;

pub struct Line {
    original: String,
    first_digit: u8,
    last_digit: u8,
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let original = s.to_owned();
        let first_digit = original.chars().find(|x| x.is_ascii_digit());
        let last_digit = original.chars().rfind(|x| x.is_ascii_digit());
        Ok(Self {
            original,
            last_digit: last_digit.and_then(|x| x.to_digit(10)).unwrap_or(99) as u8,
            first_digit: first_digit.and_then(|x| x.to_digit(10)).unwrap_or(99) as u8,
        })
    }
}

impl Line {
    fn digit_sum(&self) -> u8 {
        assert!(self.first_digit < 10);
        assert!(self.last_digit < 10);
        self.first_digit * 10 + self.last_digit
    }
    fn transform_str_digits(self) -> Self {
        let original = self
            .original
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        Self::from_str(&original).unwrap()
    }
}

impl Solution for S {
    type InputType = Vec<Line>;
    type InputParser = Lines<Line>;
    type Unpacker = PlainUnpacker;
    type Output = u64;
    type OutputResult = u64;

    const PUZZLE: (Year, Day) = (year(2023), day(1));

    fn run_part1(inp: Self::InputType, _: ()) -> Result<u64> {
        Ok(inp.into_iter().map(|x| x.digit_sum() as u64).sum())
    }

    fn run_part2(inp: Self::InputType, _: ()) -> Result<u64> {
        Ok(inp
            .into_iter()
            .map(|x| x.transform_str_digits().digit_sum() as u64)
            .sum())
    }
}
