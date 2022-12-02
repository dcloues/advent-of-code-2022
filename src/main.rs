mod days;

use days::day1;
use std::env::{args, current_dir};
use std::error::Error;
use std::path::PathBuf;

fn default_input(day: i32, relative: &PathBuf) -> std::io::Result<String> {
    let path = relative.join("input").join(format!("day{}.txt", day));
    std::fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    match args.len() {
        3 => {
            let day: i32 = args[1].parse()?;
            let part: i32 = args[2].parse()?;
            let input: String = default_input(day, &current_dir()?)?;

            let result = match part {
                1 => crate::day1::part1(&input),
                2 => crate::day1::part2(&input),
                _ => Err("invalid part (must be 1 or 2".into()),
            }?;
            println!("Day {} Part {}: {}", day, part, result);

            Ok(())
        }
        _ => {
            println!("Usage: adventofcode2022 day");
            Err("invalid arguments".into())
        }
    }
}
