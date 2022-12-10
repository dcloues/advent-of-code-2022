use std::{error::Error, iter::repeat, mem::replace, str::FromStr};

enum Instruction {
    Noop,
    AddX(i32),
}

#[derive(Debug)]
enum Stage {
    Noop,
    BeginAddX(i32),
    CommitAddX(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts.as_slice() {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", by] => Ok(Instruction::AddX(by.parse()?)),
            _ => Err(format!("malformed instruction '{s}'").into()),
        }
    }
}

impl Instruction {
    fn stages(&self) -> Vec<Stage> {
        match self {
            Instruction::Noop => vec![Stage::Noop],
            Instruction::AddX(n) => vec![Stage::BeginAddX(*n), Stage::CommitAddX(*n)],
        }
    }
}

struct Processor<'a> {
    x: i32,
    counter: i32,
    instructions: Box<dyn Iterator<Item = Stage> + 'a>,
}

impl<'a> Processor<'a> {
    fn new(instructions: &'a Vec<Instruction>) -> Self {
        Self {
            x: 1,
            counter: 0,
            instructions: Box::new(instructions.iter().flat_map(|i| i.stages())),
        }
    }
}

impl<'a> Iterator for Processor<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        self.counter += 1;
        match self.instructions.next() {
            Some(Stage::CommitAddX(n)) => Some((self.counter, replace(&mut self.x, x + n))),
            Some(Stage::BeginAddX(_) | Stage::Noop) => Some((self.counter, x)),
            None => None,
        }
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(<Instruction as FromStr>::from_str)
        .collect::<Result<_, _>>()?;

    let proc = Processor::new(&instructions);
    Ok(proc
        .skip(19)
        .step_by(40)
        .map(|(counter, x)| counter * x)
        .sum::<i32>()
        .to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(<Instruction as FromStr>::from_str)
        .collect::<Result<_, _>>()?;

    let proc = Processor::new(&instructions);
    let beam = repeat(0..40).flatten();

    let output = proc
        .zip(beam)
        .fold(String::new(), |mut acc, ((_, x), beamx)| {
            if (x - 1..=x + 1).contains(&beamx) {
                acc.push('#');
            } else {
                acc.push('.');
            }

            if beamx == 39 {
                acc.push('\n')
            }
            acc
        });

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day10test.txt");
    const OUTPUT2: &str = include_str!("tests/day10output2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "13140")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap().trim_end(), OUTPUT2);
    }
}
