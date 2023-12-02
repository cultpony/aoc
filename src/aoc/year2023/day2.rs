use std::{convert::Infallible, str::FromStr};

use crate::aoc::day::day;
use crate::aoc::year::year;
use crate::{
    add_solution, add_test,
    aoc::{day::Day, year::Year},
    util::parsing::{Lines, PlainUnpacker, Solution},
};
use anyhow::Result;

add_solution!(S : Round{ red: 12, green: 13, blue: 14 });
add_test!(S, part 1, r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"# ; Round{ red: 12, green: 13, blue: 14 } => "8");

add_test!(S, part 2, r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"# ; Round{ red: 12, green: 13, blue: 14 } => "2286");

#[derive(Debug, Clone)]
pub struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn possible(&self, required_reality: Round) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_within(&required_reality))
    }
    pub fn max_round(&self) -> Round {
        self.rounds.iter().fold(
            Round {
                red: 0,
                blue: 0,
                green: 0,
            },
            |acc, round| acc ^ *round,
        )
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let game_data = s.split_once(':').unwrap().1;

        let mut game = Game { rounds: Vec::new() };

        for round in game_data.split(';') {
            let round = round.trim();

            if let Some((first, second)) = round.split_once(',') {
                if let Some((second, third)) = second.split_once(',') {
                    let first = Round::from_str(first.trim()).unwrap();
                    let second = Round::from_str(second.trim()).unwrap();
                    let third = Round::from_str(third.trim()).unwrap();
                    game.rounds.push(first + second + third);
                } else {
                    let first = Round::from_str(first.trim()).unwrap();
                    let second = Round::from_str(second.trim()).unwrap();
                    game.rounds.push(first + second);
                }
            } else {
                game.rounds.push(Round::from_str(round.trim()).unwrap());
            }
        }

        Ok(game)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Round {
    red: u8,
    blue: u8,
    green: u8,
}

impl Round {
    pub fn power(&self) -> usize {
        (self.red as usize) * (self.green as usize) * (self.blue as usize)
    }
    fn is_within(&self, reality: &Round) -> bool {
        if self.red > reality.red || self.green > reality.green {
            false
        } else {
            self.blue <= reality.blue
        }
    }

    fn color(s: &str) -> Color {
        match s {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => unreachable!("invalid color {s}"),
        }
    }

    fn with_color(c: Color, count: u8) -> Self {
        match c {
            Color::Red => Self {
                red: count,
                ..Default::default()
            },
            Color::Green => Self {
                green: count,
                ..Default::default()
            },
            Color::Blue => Self {
                blue: count,
                ..Default::default()
            },
        }
    }
}

impl FromStr for Round {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, color) = s.split_once(' ').unwrap();
        let color = Self::color(color);
        let count: u8 = count.parse().unwrap();
        Ok(Self::with_color(color, count))
    }
}

impl std::ops::Add for Round {
    type Output = Round;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::ops::BitXor for Round {
    type Output = Round;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}

pub struct S;

impl Solution for S {
    type InputType = Vec<Game>;
    type InputParser = Lines<Game>;
    type Unpacker = PlainUnpacker;
    type Output = usize;
    type OutputResult = usize;
    type AdditionalArguments = Round;

    const PUZZLE: (Year, Day) = (year(2023), day(2));

    fn run_part1(inp: Self::InputType, required_reality: Round) -> Result<usize> {
        Ok(inp
            .iter()
            .enumerate()
            .map(|(id, game)| (id + 1, game))
            .filter(|(_, game)| game.possible(required_reality))
            .fold(0usize, |acc, (id, _)| acc + id))
    }

    fn run_part2(inp: Self::InputType, _: Round) -> Result<usize> {
        Ok(inp
            .iter()
            .enumerate()
            .map(|(id, game)| (id + 1, game))
            .map(|(_, game)| game.max_round().power())
            .fold(0usize, usize::saturating_add))
    }
}
