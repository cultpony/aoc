use std::{convert::Infallible, str::FromStr};

use crate::aoc::day::day;
use crate::aoc::year::year;
use crate::{
    add_solution, add_test,
    aoc::{day::Day, year::Year},
    util::parsing::{Lines, PlainUnpacker, Solution},
};
use anyhow::Result;
use itertools::Itertools;

add_solution!(S);
add_test!(S, part 1, r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"# => "13");
add_test!(S, part 2, r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"# => "30");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    id: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
    super_value: usize,
}

impl Card {
    pub fn points(&self) -> usize {
        let mut points = 0;
        for win in &self.winning {
            if self.have.contains(win) {
                points = (points * 2).max(1);
            }
        }
        points
    }
    pub fn wins(&self) -> usize {
        let mut points = 0;
        for win in &self.winning {
            if self.have.contains(win) {
                points += 1;
            }
        }
        points
    }
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let regex =
            regex::Regex::new(r#"^Card\s+(?P<id>\d+):(?P<win>(\s+\d+)+)\s\|(?P<hav>(\s+\d+)+)$"#)
                .unwrap();
        let caps = regex.captures(s).unwrap();
        let id: usize = caps.name("id").unwrap().as_str().trim().parse().unwrap();
        let winning: Vec<usize> = caps
            .name("win")
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse().unwrap())
            .collect_vec();
        let have: Vec<usize> = caps
            .name("hav")
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse().unwrap())
            .collect_vec();
        Ok(Self {
            id,
            winning,
            have,
            super_value: 1,
        })
    }
}

pub struct S;

impl Solution for S {
    type InputType = Vec<Card>;
    type InputParser = Lines<Card>;
    type Unpacker = PlainUnpacker;
    type Output = usize;
    type OutputResult = usize;

    const PUZZLE: (Year, Day) = (year(2023), day(4));

    fn run_part1(inp: Self::InputType, _: ()) -> Result<usize> {
        Ok(inp.iter().map(|x| x.points()).sum())
    }

    fn run_part2(mut inp: Self::InputType, _: ()) -> Result<usize> {
        let mut i = 0;
        while i < inp.len() {
            let card = inp[i].clone();
            let wins = card.wins();
            let super_value = card.super_value;
            let mut off = 1;
            for _ in 0..wins {
                inp[card.id + off - 1].super_value += super_value;
                off += 1;
            }
            i += 1;
        }
        Ok(inp.iter().map(|x| x.super_value).sum())
    }
}
