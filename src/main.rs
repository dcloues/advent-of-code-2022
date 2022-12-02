mod days;

use days::day1;
use std::env::{args, current_dir};
use std::error::Error;
use std::path::PathBuf;

fn default_input(day: usize, relative: &PathBuf) -> std::io::Result<String> {
    let path = relative.join("input").join(format!("day{}.txt", day));
    std::fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    type Part = dyn Fn(&str) -> Result<String, Box<dyn Error>>;
    let days: Vec<Vec<Box<Part>>> = vec![vec![Box::new(day1::part1), Box::new(day1::part2)]];

    match args.len() {
        3 => {
            let day: usize = args[1].parse()?;
            let part: usize = args[2].parse()?;
            let input: String = default_input(day, &current_dir()?)?;

            let f = days
                .get(day - 1)
                .ok_or(format!("invalid day {}", day))?
                .get(part - 1)
                .ok_or(format!("invalid part {} for day {}", part, day))?;

            let result = f(&input)?;
            println!("Day {} Part {}: {}", day, part, result);
            Ok(())
        }
        _ => {
            println!("Usage: adventofcode2022 day");
            Err("invalid arguments".into())
        }
    }
}
