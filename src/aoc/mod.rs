mod year2019;
mod year2023;

pub mod day;
mod exec;
pub mod part;
pub mod part_status;
pub mod progress_report;
mod puzzle;
mod puzzle_test_input;
pub mod puzzlespec;
pub mod year;

pub use exec::*;
pub use puzzle::puzzle;
pub use puzzle_test_input::puzzle_test;

use self::{puzzle::Puzzle, puzzle_test_input::PuzzleTestInput};

inventory::collect!(Puzzle);
inventory::collect!(PuzzleTestInput);
