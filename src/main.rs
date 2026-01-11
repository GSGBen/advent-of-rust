use core::panic;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod shared;
mod year2025;

fn main() {
    let args = parse_args();

    let puzzle_input = get_puzzle_input(&args);

    match (args.year.as_ref(), args.day.as_ref(), args.puzzle.as_ref()) {
        ("2025", "01", "01") => year2025::day01::puzzle01(puzzle_input),
        _ => println!(
            "no solution written for year {}, day {}, puzzle {}",
            args.year, args.day, args.puzzle
        ),
    }
}

struct Args {
    year: String,
    day: String,
    puzzle: String,
}

fn parse_args() -> Args {
    let mut args = env::args();
    let proc_name = args.next().expect("couldn't even get first argv");
    let usage = format!(
        "
usage:
    {} <year num: e.g 2025> <day num: 01-12 or 01-25> <day puzzle: 01-02>
from the directory with input/<year>/<day>_<day puzzle>.txt in it.
",
        proc_name
    );
    let year = args.next().expect(&format!("not enough args.\n{}", usage));
    let day = args.next().expect(&format!("not enough args.\n{}", usage));
    let puzzle = args.next().expect(&format!("not enough args.\n{}", usage));
    Args { year, day, puzzle }
}

fn get_puzzle_input(args: &Args) -> Vec<String> {
    let input_path = env::current_dir()
        .expect("couldn't get the current working directory?")
        .join("input")
        .join(&args.year)
        .join(format!("{}_{}.txt", args.day, args.puzzle));

    let file = File::open(&input_path).expect(&format!("couldn't open path {:#?}", input_path));
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .map(|(i, maybe_line)| match maybe_line {
            Ok(line) => line,
            Err(err) => panic!("couldn't parse line at index {}. Error: {}", i, err),
        })
        .collect()
}
