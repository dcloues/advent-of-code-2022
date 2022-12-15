use std::{error::Error, str::FromStr};

use crate::grid::{Grid, Location};

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
enum Material {
    #[default]
    Empty,
    Stone,
    Sand,
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    AtRest,
    Escaped,
    Blocked,
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

#[must_use]
fn run_sand(grid: &mut Grid<Material>, mut sand: Location) -> Outcome {
    if let Some(Material::Sand) = grid.get(sand) {
        return Outcome::Blocked;
    }

    grid.set(sand, Material::Sand);

    loop {
        // first, figure out which of the move options (down, down+left, down+right)
        // are valid, and pass along the material type encountered at the first
        // valid move.
        let mv = [sand.up(), sand.up().left(), sand.up().right()]
            .iter()
            .cloned()
            .find_map(|l| {
                let m = grid.get(l);
                match m {
                    None | Some(Material::Empty) => Some((l, m)),
                    _ => None,
                }
            });

        match mv {
            // If the material type is None, we fell off the grid
            Some((_, None)) => return Outcome::Escaped,
            // Otherwise, move the sand into place and repeat
            Some((to, Some(Material::Empty))) => {
                grid.set(sand, Material::Empty);
                grid.set(to, Material::Sand);
                sand = to;
            }
            // We couldn't find a move at all
            None => return Outcome::AtRest,
            // This is a logic error - the move selection, above, should never produce this
            Some(_) => panic!("impossible move state reached: {mv:?}"),
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
