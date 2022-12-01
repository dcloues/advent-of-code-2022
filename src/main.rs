mod days;

use std::env::{self, current_dir};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use days::day1;

fn default_input(day: i32, relative: &PathBuf) -> std::io::Result<File> {
    let path = relative.join("input").join(format!("day{}.txt", day));

    return File::open(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let day: i32 = args[1].parse()?;
            let input: File = default_input(day, &current_dir()?)?;

            let result = crate::day1::run(input)?;
            println!("Day {}: {}", day, result);

            Ok(())
        },
        _ => {
            println!("Usage: adventofcode2022 day");
            Err("invalid arguments".into())
        }
    }

}
