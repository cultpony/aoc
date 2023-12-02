#![feature(never_type)]
#![feature(const_trait_impl)]
#![feature(associated_type_defaults)]

pub mod aoc;
mod util;

use std::panic::catch_unwind;

use aoc::get_puzzles;
use clap::Parser;
use util::{Action, Arguments};

use crate::aoc::{part_status::PartStatus, run_self_test};

#[macro_export]
macro_rules! add_test {
    (internal : $sol:ty, $part:expr ; $add:expr, $inp:literal => $out:literal) => {
        inventory::submit!{
            $crate::aoc::puzzle_test::<$sol>(
                <$sol as Solution>::PUZZLE.0,
                <$sol as Solution>::PUZZLE.1,
                $part,
                $inp,
                $add,
                $out,
            )
        }
    };
    ($sol:ty, part 1, $inp:literal => $out:literal) => {
        add_test!(internal : $sol, $crate::aoc::part::Part::one() ; &(), $inp => $out);
    };
    ($sol:ty, part 2, $inp:literal => $out:literal) => {
        add_test!(internal : $sol, $crate::aoc::part::Part::two() ; &(), $inp => $out);
    };
    ($sol:ty, all parts, $inp:literal =>  $out:literal) => {
        add_test!($sol, part 1, $inp => $out);
        add_test!($sol, part 2, $inp => $out);
    };
    ($sol:ty, part 1, $inp:literal ; $add:expr => $out:literal) => {
        add_test!(internal : $sol, $crate::aoc::part::Part::one() ; &$add, $inp => $out);
    };
    ($sol:ty, part 2, $inp:literal ; $add:expr => $out:literal) => {
        add_test!(internal : $sol, $crate::aoc::part::Part::two() ; &$add, $inp => $out);
    };
    ($sol:ty, all parts, $inp:literal ; $add:expr =>  $out:literal) => {
        add_test!($sol, part 1, $inp ; &$add => $out);
        add_test!($sol, part 2, $inp ; &$add => $out);
    };
}

#[macro_export]
macro_rules! add_solution {
    (internal $sol:ty : $part:expr ; $add:expr, $fun:ident) => {
        inventory::submit! {
            $crate::aoc::puzzle::<$sol>(
                <$sol as Solution>::PUZZLE.0,
                <$sol as Solution>::PUZZLE.1,
                $part,
                &$add,
                &|inp: &str, add: *const u8| -> anyhow::Result<String> {
                    use anyhow::Context;
                    use std::str::FromStr;
                    use $crate::util::parsing::Unpackable;
                    use $crate::util::parsing::Unpacker;
                    use $crate::util::parsing::ResultUnpacker;

                    let add = add as *const <$sol as Solution>::AdditionalArguments;
                    let add: &<$sol as Solution>::AdditionalArguments = unsafe { &*add };

                    let inp = <$sol as Solution>::InputParser::from_str(inp)
                        .context("parsing puzzle input failed")?;

                    let data = inp.unpack();

                    let data = <$sol as Solution>::Unpacker::unpacked(data);

                    let out = <$sol as Solution>::$fun(data, *add)
                        .context("puzzle execution failed")?;

                    let out: <$sol as Solution>::Output = <$sol as Solution>::OutputResult::unpack(out);
                    Ok(out.to_string())
                }
            )
        }
    };
    ($sol:ty) => {
        add_solution!(internal $sol : $crate::aoc::part::Part::one() ; () , run_part1);
        add_solution!(internal $sol : $crate::aoc::part::Part::two() ; () , run_part2);
    };
    ($sol:ty : $add:expr) => {
        add_solution!(internal $sol : $crate::aoc::part::Part::one() ; $add , run_part1);
        add_solution!(internal $sol : $crate::aoc::part::Part::two() ; $add , run_part2);
    };
}

fn main() -> anyhow::Result<()> {
    let arg = Arguments::parse();
    if !run_self_test(arg.puzzle).is_empty() {
        println!("Fix failing tests before submitting more puzzle solutions");
        anyhow::bail!("self-test failure")
    }
    match arg.action {
        Action::ListAoC => {
            for puzzle in get_puzzles(arg.puzzle) {
                println!(
                    "Year {}, Day {}, Part {} = {}",
                    puzzle.year(),
                    puzzle.day(),
                    puzzle.part(),
                    arg.get_day_progress(puzzle.year(), puzzle.day(), puzzle.part())?
                        .status()
                );
            }
        }
        Action::Benchmark => {
            for puzzle in get_puzzles(arg.puzzle) {
                use anyhow::Context;
                let input = arg
                .get_input(puzzle.year(), puzzle.day())
                .context("puzzle input gathering")?;
                let start = std::time::Instant::now();
                for _ in 0..arg.bench_loops() {
                    let output = catch_unwind(|| -> anyhow::Result<String> {
                        let solution = puzzle.call(&input, None).context("puzzle call")?;
                        Ok(solution)
                    });
                    match output {
                        Ok(Ok(solution)) => {
                            let _ = std::hint::black_box(solution);
                        }
                        Ok(Err(e)) => {
                            println!("Puzzle {puzzle} encountered an error: {e:?}");
                            break;
                        }
                        Err(_) => {
                            println!("Puzzle {puzzle} panicked");
                            break;
                        }
                    }
                }
                println!(
                    "Puzzle {puzzle} takes {:05.4} µs per invocation",
                    start.elapsed().as_secs_f64() * 1_000_000.0 / (arg.bench_loops() as f64)
                );
            }
            println!("DONE");
        }
        Action::Run => {
            for puzzle in get_puzzles(arg.puzzle) {
                let progress = arg.get_day_progress(puzzle.year(), puzzle.day(), puzzle.part())?;
                if progress.status() == PartStatus::Completed {
                    // skip days we finished
                    continue;
                }
                let output = catch_unwind(|| -> anyhow::Result<String> {
                    use anyhow::Context;
                    let input = arg
                        .get_input(puzzle.year(), puzzle.day())
                        .context("puzzle input gathering")?;
                    let solution = puzzle.call(&input, None).context("puzzle call")?;
                    Ok(solution)
                });
                match output {
                    Ok(Ok(solution)) => {
                        println!("Solution for {puzzle} = {solution}");
                        println!(
                            "Submission Result: {:?}",
                            arg.submit_solution(
                                puzzle.year(),
                                puzzle.day(),
                                puzzle.part(),
                                solution
                            )?
                        );
                    }
                    Ok(Err(e)) => println!("Puzzle {puzzle} encountered an error: {e:?}"),
                    Err(_) => println!("Puzzle {puzzle} panicked"),
                }
            }
            println!("DONE");
        }
    }

    Ok(())
}
