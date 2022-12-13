mod days;

use days::day1;
use days::day10;
use days::day11;
use days::day12;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
use days::day6;
use days::day7;
use days::day8;
use days::day9;
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
    let days: Vec<Vec<Box<Part>>> = vec![
        vec![Box::new(day1::part1), Box::new(day1::part2)],
        vec![Box::new(day2::part1), Box::new(day2::part2)],
        vec![Box::new(day3::part1), Box::new(day3::part2)],
        vec![Box::new(day4::part1), Box::new(day4::part2)],
        vec![Box::new(day5::part1), Box::new(day5::part2)],
        vec![Box::new(day6::part1), Box::new(day6::part2)],
        vec![Box::new(day7::part1), Box::new(day7::part2)],
        vec![Box::new(day8::part1), Box::new(day8::part2)],
        vec![Box::new(day9::part1), Box::new(day9::part2)],
        vec![Box::new(day10::part1), Box::new(day10::part2)],
        vec![Box::new(day11::part1), Box::new(day11::part2)],
        vec![Box::new(day12::part1), Box::new(day12::part2)],
    ];

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
            println!("Day {} Part {}:\n{}", day, part, result);
            Ok(())
        }
        _ => {
            println!("Usage: adventofcode2022 day");
            Err("invalid arguments".into())
        }
    }
}
