use std::{convert::Infallible, str::FromStr};

use crate::aoc::day::day;
use crate::aoc::year::year;
use crate::util::parsing::Unpackable;
use crate::{
    add_solution, add_test,
    aoc::{day::Day, year::Year},
    util::parsing::{PlainUnpacker, Solution},
};
use anyhow::Result;
use itertools::Itertools;

add_solution!(S);

add_test!(S, part 1, r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."# => "4361");

add_test!(S, part 2, r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."# => "467835");

#[derive(Clone)]
pub struct Blueprint {
    data: nalgebra::DMatrix<BlueprintItem>,
}

impl Unpackable<Blueprint> for Blueprint {
    fn unpack(self) -> Blueprint {
        self
    }
}

impl FromStr for Blueprint {
    type Err = Infallible;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let width = s.lines().find(|x| !x.trim().is_empty()).unwrap().len();
        let height = s.lines().filter(|x| !x.trim().is_empty()).count();
        let mut data = nalgebra::DMatrix::from_element(height, width, BlueprintItem::default());
        for (i, line) in s.lines().filter(|x| !x.trim().is_empty()).enumerate() {
            for (j, char) in line.chars().enumerate() {
                let el = char.to_string().parse().unwrap();
                data.row_mut(i)[j] = el;
            }
        }
        let mut data = Self { data };
        data.fold_numbers();
        Ok(data)
    }
}

impl std::fmt::Debug for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data: Vec<Vec<String>> = Vec::new();
        for i in 0..self.data.nrows() {
            let mut row: Vec<String> = Vec::new();
            for j in 0..self.data.ncols() {
                row.push(format!("{:?}", self.get_at_true(i, j)))
            }
            data.push(row);
        }
        f.debug_struct("Blueprint").field("data", &data).finish()
    }
}

impl Blueprint {
    pub fn fold_numbers(&mut self) {
        for i in 0..self.data.nrows() {
            for j in 0..self.data.ncols() {
                let el = self.get_at_true(i, j);
                if let BlueprintItem::Number(q) = el {
                    if j < self.data.ncols() - 1 {
                        let n = self.get_at_true(i, j + 1);
                        if let BlueprintItem::Number(n) = n
                            && j < self.data.ncols() - 2
                        {
                            if let BlueprintItem::Number(nd) = self.get_at_true(i, j + 2) {
                                let q = q * 100 + n * 10 + nd;
                                self.set_at(i, j, BlueprintItem::Number(q));
                                self.set_at(i, j + 1, BlueprintItem::NumberPlaceholder);
                                self.set_at(i, j + 2, BlueprintItem::NumberPlaceholder);
                                continue;
                            }
                            let q = q * 10 + n;
                            self.set_at(i, j, BlueprintItem::Number(q));
                            self.set_at(i, j + 1, BlueprintItem::NumberPlaceholder);
                        }
                    }
                }
            }
        }
        for row in 0..self.data.nrows() {
            for col in 0..self.data.ncols() {
                let (_, el) = self.get_at(row, col);
                match el {
                    BlueprintItem::Number(x) => {
                        let nbs = self.get_nb(row, col);
                        let has_symbol = nbs
                            .into_iter()
                            .map(|(x, y)| self.get_at_true(x, y))
                            .any(|x| matches!(x, BlueprintItem::Part(_)));
                        if has_symbol {
                            self.set_at_indirect(row, col, BlueprintItem::MarkedNumber(x));
                        }
                    }
                    BlueprintItem::NumberPlaceholder => {
                        let x = if let (_, BlueprintItem::Number(x)) = self.get_at(row, col) {
                            x
                        } else { continue };
                        let nbs = self.get_nb(row, col);
                        let has_symbol = nbs
                            .into_iter()
                            .map(|(x, y)| self.get_at_true(x, y))
                            .any(|x| matches!(x, BlueprintItem::Part(_)));
                        if has_symbol {
                            self.set_at_indirect(row, col, BlueprintItem::MarkedNumber(x));
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    pub fn get_marked_numbers(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for row in 0..self.data.nrows() {
            for col in 0..self.data.ncols() {
                if let BlueprintItem::MarkedNumber(v) = self.get_at_true(row, col) {
                    out.push(v);
                }
            }
        }
        out
    }
    fn get_gear_pos(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for row in 0..self.data.nrows() {
            for col in 0..self.data.ncols() {
                if BlueprintItem::Part('*') == self.get_at_true(row, col) {
                    out.push((row, col))
                }
            }
        }
        out
    }
    fn get_gear_ratio(&self, row: usize, col: usize) -> Option<usize> {
        let nbs = self.get_nb(row, col);
        let ratios: Vec<BlueprintItem> = nbs.into_iter().filter_map(|(row, col)| if let ((tcol, trow), BlueprintItem::MarkedNumber(x)) = self.get_at(row, col) {
            Some(((tcol, trow), BlueprintItem::MarkedNumber(x)))
        } else { None })
            .dedup_by(|((trowa, tcola), _), ((trowb, tcolb), _)| {
                tcola == tcolb && trowa == trowb
            })
            .map(|(_, m)| m)
            .collect();
        if ratios.len() != 2 {
            return None
        }
        Some(ratios.into_iter().fold(1, |acc, x| if let BlueprintItem::MarkedNumber(x) = x {
            acc * x
        } else { unreachable!() }))
    }
    fn get_gear_ratio_sums(&self) -> usize {
        let mut out = 0;
        for (row, col) in self.get_gear_pos() {
            if let Some(sum) = self.get_gear_ratio(row, col){
                out += sum;
            }
        }
        out
    }
    pub fn get_nb(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut nbs =  Vec::new();
        let can_up = row > 0;
        let can_down = row < self.data.nrows() - 1;
        let can_left = col > 0;
        let can_right = col < self.data.ncols() - 1;
        if can_up {
            nbs.push((row - 1, col));
            if can_left {
                nbs.push((row - 1, col - 1))
            }
            if can_right {
                nbs.push((row - 1, col + 1))
            }
        }
        if can_down {
            nbs.push((row + 1, col));
            if can_left {
                nbs.push((row + 1, col - 1))
            }
            if can_right {
                nbs.push((row + 1, col + 1))
            }
        }
        if can_left {
            nbs.push((row, col - 1))
        }
        if can_right {
            nbs.push((row, col + 1))
        }
        nbs
    }

    fn get_at_true(&self, row: usize, col: usize) -> BlueprintItem {
        self.data.row(row)[col]
    }

    pub fn get_at(&self, row: usize, mut col: usize) -> ((usize, usize), BlueprintItem) {
        loop {
            let data = self.data.row(row)[col];
            match data {
                v @ BlueprintItem::MarkedNumber(_) => return ((row, col), v),
                v @ BlueprintItem::Number(_) => return ((row, col), v),
                BlueprintItem::NumberPlaceholder => {
                    col -= 1;
                }
                v => return ((row, col), v),
            }
        }
    }
    pub fn set_at(&mut self, row: usize, col: usize, data: BlueprintItem) -> BlueprintItem {
        let mut row_mut = self.data.row_mut(row);
        let old_data = row_mut[col];
        row_mut[col] = data;
        old_data
    }

    pub fn set_at_indirect(&mut self, row: usize, mut col: usize, data: BlueprintItem) -> BlueprintItem {
        loop {
            let data_n = self.data.row(row)[col];
            match data_n {
                BlueprintItem::NumberPlaceholder => {
                    col -= 1;
                }
                v => {
                    self.data.row_mut(row)[col] = data;
                    return v
                },
            }
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlueprintItem {
    Number(usize),
    MarkedNumber(usize),
    NumberPlaceholder,
    Part(char),
    #[default]
    Nothing,
}

impl FromStr for BlueprintItem {
    type Err = Infallible;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            v @ '0'..='9' => Ok(Self::Number(v.to_digit(10).unwrap() as usize)),
            '.' => Ok(Self::Nothing),
            v => Ok(Self::Part(v)),
        }
    }
}

pub struct S;

impl Solution for S {
    type InputType = Blueprint;
    type InputParser = Blueprint;
    type Unpacker = PlainUnpacker;
    type Output = usize;
    type OutputResult = usize;
    type AdditionalArguments = ();

    const PUZZLE: (Year, Day) = (year(2023), day(3));

    fn run_part1(inp: Self::InputType, _: ()) -> Result<usize> {
        Ok(inp.get_marked_numbers().iter().sum())
    }

    fn run_part2(inp: Self::InputType, _: ()) -> Result<usize> {
        Ok(inp.get_gear_ratio_sums())
    }
}
