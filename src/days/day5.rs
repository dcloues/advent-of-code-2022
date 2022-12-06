use std::{error::Error, str::FromStr};

#[derive(PartialEq, Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(PartialEq, Debug)]
struct Instructions(Vec<Instruction>);

#[derive(PartialEq, Debug)]
struct Stacks(Vec<Vec<char>>);

#[derive(PartialEq, Debug)]
struct Input {
    stacks: Stacks,
    instructions: Instructions,
}

#[derive(PartialEq, Copy, Clone)]
enum CrateGrouping {
    Batch,
    Single,
}

impl Input {
    fn process_instructions(self, order: CrateGrouping) -> Result<Stacks, Box<dyn Error>> {
        let stacks = self.stacks;
        self.instructions
            .0
            .iter()
            .enumerate()
            .try_fold(stacks, |s, (n, i)| {
                s.process(i, order)
                    .map_err(|e| format!("error '{}' on input line {}: {:?}", e, n, i).into())
            })
    }
}

impl Stacks {
    fn process(mut self, i: &Instruction, grouping: CrateGrouping) -> Result<Self, Box<dyn Error>> {
        let src = self
            .0
            .get_mut(i.from - 1)
            .ok_or_else(|| format!("invalid source {}", i.from))?;
        let mut crates: Vec<char> = src[src.len() - i.count..src.len()].into();
        if grouping == CrateGrouping::Single {
            crates.reverse();
        }
        src.truncate(src.len() - i.count);
        self.0
            .get_mut(i.to - 1)
            .ok_or_else(|| format!("invalid dest {}", i.to))?
            .extend_from_slice(&crates);

        Ok(self)
    }
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, instructions) = s
            .split_once("\n\n")
            .ok_or("invalid input - must contain two sections".to_string())?;
        Ok(Self {
            stacks: stacks.parse()?,
            instructions: instructions.parse()?,
        })
    }
}

impl FromStr for Instructions {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?,
        ))
    }
}

impl FromStr for Stacks {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let labels = lines.next().ok_or("invalid input - too short")?;

        let mut stacks = vec![vec![]; labels.chars().filter(|c| c.is_numeric()).count()];

        for line in lines {
            for (i, c) in line.char_indices().filter(|(_, c)| c.is_alphabetic()) {
                stacks[i / 4].push(c);
            }
        }

        Ok(Self(stacks))
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<&str>>().as_slice() {
            ["move", count, "from", from, "to", to] => Ok(Self {
                from: from.parse()?,
                to: to.parse()?,
                count: count.parse()?,
            }),
            _ => Err(format!("invalid instruction '{}'", s).into()),
        }
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Input = input.parse()?;
    let output = input.process_instructions(CrateGrouping::Single)?;

    Ok(output.0.iter().filter_map(|s| s.last()).collect())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Input = input.parse()?;
    let output = input.process_instructions(CrateGrouping::Batch)?;

    Ok(output.0.iter().filter_map(|s| s.last()).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day5test.txt");

    #[test]
    fn test_instruction_fromstr() {
        assert_eq!(
            Instruction {
                from: 1,
                to: 19,
                count: 5
            },
            "move 5 from 1 to 19".parse().unwrap(),
        );
    }

    #[test]
    fn test_full_parse() {
        let input: Input = INPUT.parse().unwrap();
        assert_eq!(
            input,
            Input {
                stacks: Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],],),
                instructions: Instructions(Vec::from([
                    Instruction {
                        count: 1,
                        from: 2,
                        to: 1
                    },
                    Instruction {
                        count: 3,
                        from: 1,
                        to: 3
                    },
                    Instruction {
                        count: 2,
                        from: 2,
                        to: 1
                    },
                    Instruction {
                        count: 1,
                        from: 1,
                        to: 2
                    },
                ]))
            },
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "CMZ")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "MCD")
    }
}
