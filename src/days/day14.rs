use std::{
    error::Error,
    fmt::{Display, Write},
    str::FromStr,
};

use crate::grid::{Grid, Location};

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Empty => f.write_char('.'),
            Material::Stone => f.write_char('#'),
            Material::Sand => f.write_char('o'),
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
enum Material {
    #[default]
    Empty,
    Stone,
    Sand,
}

fn parse_line(line: &str) -> Result<Vec<Location>, Box<dyn Error>> {
    line.split(" -> ").map(Location::from_str).collect()
}

fn parse_input(input: &str) -> Result<Grid<Material>, Box<dyn Error>> {
    let shapes: Vec<Vec<Location>> = input.lines().map(parse_line).collect::<Result<_, _>>()?;
    let mut grid = Grid::new(
        shapes.iter().flatten().map(|l| l.col).max().unwrap() + 1,
        shapes.iter().flatten().map(|l| l.row).max().unwrap() + 1,
    );

    for shape in shapes {
        for (start, end) in shape.iter().zip(shape.iter().skip(1)) {
            for loc in start.to(end).ok_or_else(|| -> Box<dyn Error> {
                format!("bad shape: {start:?} -> {end:?}").into()
            })? {
                grid.set(loc, Material::Stone);
            }
        }
    }
    Ok(grid)
}

enum Move {
    Moved,
    Escaped,
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    AtRest,
    Escaped,
    Blocked,
}

fn try_move(grid: &mut Grid<Material>, from: Location, to: Location) -> Option<Move> {
    match grid.get(to) {
        None => Some(Move::Escaped),
        Some(Material::Empty) => {
            grid.set(from, Material::Empty);
            grid.set(to, Material::Sand);
            Some(Move::Moved)
        }
        _ => None,
    }
}

#[must_use]
fn run_sand(grid: &mut Grid<Material>, mut sand: Location) -> Outcome {
    if let Some(Material::Sand) = grid.get(sand) {
        return Outcome::Blocked;
    }

    grid.set(sand, Material::Sand);

    loop {
        let options = [sand.up(), sand.up().left(), sand.up().right()];

        let mut moved = false;
        for to in options {
            match try_move(grid, sand, to) {
                Some(Move::Moved) => {
                    sand = to;
                    moved = true;
                    break;
                }
                Some(Move::Escaped) => return Outcome::Escaped,
                None => {}
            }
        }
        if !moved {
            return Outcome::AtRest;
        }
    }
}

fn run_sand_until(mut grid: Grid<Material>, halt_on: Outcome) -> i32 {
    let mut count = 0;
    loop {
        match run_sand(&mut grid, Location::new(0, 500)) {
            Outcome::AtRest => count += 1,
            o if o == halt_on => return count,
            s => panic!("impossible outcome {s:?}"),
        }
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = parse_input(input)?;
    return Ok(run_sand_until(grid, Outcome::Escaped).to_string());
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut grid = parse_input(input)?;

    let base = grid.locations().map(|l| l.row).max().unwrap() + 2;
    grid.expand(base + 1, 1000);
    for col in 0..1000 {
        grid.set(Location::new(base, col), Material::Stone);
    }

    return Ok(run_sand_until(grid, Outcome::Blocked).to_string());
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day14test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "24")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "93")
    }
}
