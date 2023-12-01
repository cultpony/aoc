use super::{day::Day, part::Part, puzzlespec::PuzzleSpec, year::Year, Puzzle, PuzzleTestInput};

pub fn get_puzzle_test_inputs(puzzle: &Puzzle) -> impl Iterator<Item = PuzzleTestInput> {
    itertools::sorted(
        inventory::iter::<PuzzleTestInput>
            .into_iter()
            .cloned()
            .filter_map(move |p| if p == *puzzle { Some(p) } else { None }),
    )
}

pub fn get_puzzles(spec: Option<PuzzleSpec>) -> impl Iterator<Item = Puzzle> {
    let spec = spec.unwrap_or_default();
    itertools::sorted(
        inventory::iter::<Puzzle>
            .into_iter()
            .cloned()
            .filter_map(move |p| if spec == p { Some(p) } else { None }),
    )
}

pub fn get_puzzle<Y: Into<Year>, D: Into<Day>, P: Into<Part>>(
    year: Y,
    day: D,
    part: P,
) -> Option<Puzzle> {
    let year = year.into();
    let day = day.into();
    let part = part.into();
    get_puzzles(None).find(|x| x.year() == year && x.day() == day && x.part() == part)
}

/// Runs all registered Puzzle Self-Tests and outputs a list of puzzles that failed their self-test
pub fn run_self_test(spec: Option<PuzzleSpec>) -> Vec<(Puzzle, PuzzleTestInput)> {
    let mut wrong = Vec::new();
    for puzzle in get_puzzles(spec) {
        for test in get_puzzle_test_inputs(&puzzle) {
            match puzzle.call(test.input(), Some(test.add())) {
                Ok(out) if out == test.output() => {
                    // good, all checks out
                }
                Ok(out) => {
                    println!("Puzzle {puzzle} failed self test: Got {out:?} instead of {:?} from input {:?}", test.output(), test.input());
                    wrong.push((puzzle.clone(), test))
                }
                Err(e) => {
                    println!("Puzzle {puzzle} error'd during self-test: {e:?}");
                    wrong.push((puzzle.clone(), test))
                }
            }
        }
    }
    wrong
}
