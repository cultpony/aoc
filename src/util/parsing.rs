use anyhow::Result;
use std::str::FromStr;

use crate::aoc::{day::Day, year::Year};

pub trait Solution {
    type InputType;
    type InputParser: Unpackable<Self::UnpackType>;
    type UnpackType = Self::InputType;
    type Unpacker: Unpacker<Self::UnpackType, Self::InputType>;
    type Output: std::fmt::Display;
    type AdditionalArguments = ();
    type DebugOutputType = ();

    type OutputResult: ResultUnpacker<Self::Output> = (Self::Output, Self::DebugOutputType);

    const PUZZLE: (Year, Day);

    fn run_part1(
        _inp: Self::InputType,
        _add: Self::AdditionalArguments,
    ) -> anyhow::Result<Self::OutputResult> {
        anyhow::bail!("not implemented")
    }
    fn run_part2(
        _inp: Self::InputType,
        _add: Self::AdditionalArguments,
    ) -> anyhow::Result<Self::OutputResult> {
        anyhow::bail!("not implemented")
    }
}

pub trait Unpacker<IN, OUT> {
    fn unpacked(inp: IN) -> OUT;
}

pub trait ResultUnpacker<T> {
    fn unpack(inp: Self) -> T;
}

impl<T, Q> ResultUnpacker<T> for (T, Q) {
    fn unpack(inp: Self) -> T {
        inp.0
    }
}

impl<T> ResultUnpacker<T> for T {
    fn unpack(inp: Self) -> T {
        inp
    }
}

pub struct PlainUnpacker;

impl<IN, OUT> Unpacker<IN, OUT> for PlainUnpacker
where
    IN: Into<OUT>,
{
    fn unpacked(inp: IN) -> OUT {
        inp.into()
    }
}

pub struct VecUnpacker;

impl<IN, OUT> Unpacker<Vec<IN>, Vec<OUT>> for VecUnpacker
where
    IN: Unpackable<OUT>,
{
    fn unpacked(inp: Vec<IN>) -> Vec<OUT> {
        inp.into_iter().map(|x| x.unpack()).collect()
    }
}

pub trait Unpackable<T>: FromStr {
    fn unpack(self) -> T;
}

pub struct Lines<T: FromStr> {
    data: Vec<T>,
}

impl<T: FromStr> FromStr for Lines<T>
where
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Result<Vec<T>, T::Err> = s.lines().map(|f| f.parse()).collect();
        let data = data?;
        Ok(Self { data })
    }
}

impl<T: FromStr> Unpackable<Vec<T>> for Lines<T>
where
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    fn unpack(self) -> Vec<T> {
        self.data
    }
}

/// Provides Comma-Separated-Value parsing, defaults to splitting at the `,`character but can be configured differently.
///
#[allow(clippy::upper_case_acronyms)]
pub struct CSV<T: FromStr, const SPLIT: char = ','> {
    data: Vec<T>,
}

impl<const SPLIT: char, T: FromStr> FromStr for CSV<T, SPLIT>
where
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let data: Result<Vec<T>, T::Err> = s.split(SPLIT).map(|f| f.parse()).collect();
        let data = data?;
        Ok(Self { data })
    }
}

impl<const SPLIT: char, T: FromStr> Unpackable<Vec<T>> for CSV<T, SPLIT>
where
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    fn unpack(self) -> Vec<T> {
        self.data
    }
}

pub struct Trimmed<T: FromStr> {
    data: T,
}

impl<T: FromStr> FromStr for Trimmed<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self {
            data: s.trim().parse()?,
        })
    }
}

impl<T: FromStr> Unpackable<T> for Trimmed<T> {
    fn unpack(self) -> T {
        self.data
    }
}
