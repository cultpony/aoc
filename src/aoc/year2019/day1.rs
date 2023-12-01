use crate::aoc::day::day;
use crate::aoc::year::year;
use crate::{
    add_solution, add_test,
    aoc::{day::Day, year::Year},
    util::parsing::{Lines, PlainUnpacker, Solution},
};
use anyhow::Result;

add_solution!(S);
add_test!(S, all parts, "12" => "2");
add_test!(S, all parts, "14" => "2");
add_test!(S, part 1, "1969" => "654");
add_test!(S, part 1, "100756" => "33583");
add_test!(S, part 2, "1969" => "966");
add_test!(S, part 2, "100756" => "50346");

pub struct S;

impl Solution for S {
    type InputType = Vec<f64>;
    type InputParser = Lines<f64>;
    type Unpacker = PlainUnpacker;
    type Output = u64;
    type OutputResult = u64;

    const PUZZLE: (Year, Day) = (year(2019), day(1));

    fn run_part1(inp: Self::InputType, _: ()) -> Result<u64> {
        Ok(masses_to_fuel(inp.into_iter()) as u64)
    }

    fn run_part2(inp: Self::InputType, _: ()) -> Result<u64> {
        Ok(rec_masses_to_fuel(inp.into_iter()) as u64)
    }
}

fn mass_to_fuel(inp: f64) -> f64 {
    ((inp / 3.0).trunc() - 2.0).max(0.0)
}

fn masses_to_fuel<I: Iterator<Item = f64>>(inp: I) -> f64 {
    inp.map(mass_to_fuel).sum()
}

fn rec_mass_to_fuel(inp: f64) -> f64 {
    let mut inp = ((inp / 3.0).trunc() - 2.0).max(0.0);
    let mut new_fuel = mass_to_fuel(inp);
    loop {
        if new_fuel <= 0.0 {
            break inp;
        }
        let prev_fuel = new_fuel;
        inp += new_fuel;
        new_fuel = mass_to_fuel(prev_fuel);
    }
}

fn rec_masses_to_fuel<I: Iterator<Item = f64>>(inp: I) -> f64 {
    inp.map(rec_mass_to_fuel).sum()
}

#[cfg(test)]
mod test {
    use crate::aoc::year2019::day1::rec_masses_to_fuel;

    #[test]
    pub fn test_recursive_m2f() {
        assert_eq!(rec_masses_to_fuel(vec![100756.0].into_iter()), 50346.0)
    }
}
