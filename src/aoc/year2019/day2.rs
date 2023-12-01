use std::fmt::Display;

use crate::{
    add_solution, add_test,
    aoc::{
        day::{day, Day},
        year::{year, Year},
    },
    util::parsing::{Solution, Trimmed, VecUnpacker, CSV},
};

use anyhow::{Context, Result};

add_solution!(S : -1202);
add_test!(S, part 1, "1,0,0,0,99" ; 0 => "2");
add_test!(S, part 1, "2,3,0,3,99" ; 3 => "6");
add_test!(S, part 1, "1,1,1,4,99,5,6,0,99" ; 0 => "30");
add_test!(S, part 1, "1,9,10,3,2,3,11,0,99,30,40,50" ; 0 => "3500");

#[derive(Default, Debug)]
pub struct S {
    memory: Vec<i64>,
    pc: isize,
    // Source A operand
    op_a: i64,
    // Source A address
    op_aa: i64,
    // Source B operand
    op_b: i64,
    // Source B address
    op_ba: i64,
    // Operation Result
    op_r: i64,
    // Operation Result Address
    op_ra: i64,
}

impl Solution for S {
    type InputType = Vec<i64>;
    type InputParser = CSV<Trimmed<i64>>;

    type Unpacker = VecUnpacker;
    type UnpackType = Vec<Trimmed<i64>>;

    type Output = i64;
    type OutputResult = i64;
    type AdditionalArguments = i64;

    const PUZZLE: (Year, Day) = (year(2019), day(2));

    fn run_part1(inp: Vec<i64>, result_in: i64) -> Result<i64> {
        let mut computer = Self {
            memory: inp,
            ..Default::default()
        };

        let result_in = if result_in.is_negative() {
            computer.write(1, 12)?;
            computer.write(2, 2)?;
            0
        } else {
            result_in
        };

        computer.execute()?;

        computer.read(result_in.abs())
    }

    fn run_part2(inp: Vec<i64>, _: i64) -> Result<i64> {
        let target = 19690720;

        let (noun, verb) = || -> Result<(i64, i64)> {
            for noun in 0..100 {
                for verb in 0..100 {
                    let mut computer = Self {
                        memory: inp.clone(),
                        ..Default::default()
                    };

                    computer.write(1, noun)?;
                    computer.write(2, verb)?;

                    computer.execute()?;

                    if computer.read(0)? == target {
                        return Ok((noun, verb));
                    }
                }
            }
            anyhow::bail!("no solution found")
        }()?;

        Ok(100 * noun + verb)
    }
}

impl S {
    pub fn execute(&mut self) -> Result<()> {
        while self.pc != -1 {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        let op = self.read(self.pc).context("could not read PC")?;
        if op == 99 {
            self.pc = -1;
            return Ok(());
        }
        self.op_aa = self
            .read(self.pc + 1)
            .context("could not read A operand address")?;
        self.op_ba = self
            .read(self.pc + 2)
            .context("could not read B operand address")?;
        self.op_ra = self
            .read(self.pc + 3)
            .context("could not read O operand address")?;
        self.op_a = self
            .read(self.op_aa)
            .context("could not read operand A indirect")?;
        self.op_b = self
            .read(self.op_ba)
            .context("could not read operand B indirect")?;
        self.pc += 4;
        match op {
            1 => self.op_r = self.op_a + self.op_b,
            2 => self.op_r = self.op_a * self.op_b,
            _ => anyhow::bail!("invalid operand"),
        }
        self.write(self.op_ra, self.op_r)?;
        Ok(())
    }

    pub fn read<I: TryInto<usize> + Display + Copy>(&self, addr: I) -> Result<i64>
    where
        <I as TryInto<usize>>::Error: std::fmt::Debug + std::error::Error + Send + Sync + 'static,
    {
        let addr: usize = addr
            .try_into()
            .with_context(|| format!("invalid address conversion: {addr}"))?;
        self
            .memory
            .get(addr)
            .with_context(|| format!("could not read addr {addr}"))
            .map(|x| *x)
    }

    pub fn write<I: TryInto<usize> + Display + Copy>(&mut self, addr: I, value: i64) -> Result<()>
    where
        <I as TryInto<usize>>::Error: std::fmt::Debug + std::error::Error + Send + Sync + 'static,
    {
        let addr: usize = addr
            .try_into()
            .with_context(|| format!("invalid address conversion: {addr}"))?;
        self.memory[addr] = value;
        Ok(())
    }
}
