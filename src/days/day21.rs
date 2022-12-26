use std::{cell::RefCell, error::Error, rc::Rc, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Datum = i64;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

type ID = String;

#[derive(PartialEq)]
enum Input {
    Monkey(ID),
    Constant(Datum),
}

struct Monkey {
    id: ID,
    output: Option<Datum>,
    operation: Option<Operation>,
    input1: Option<Input>,
    input2: Option<Input>,
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(format!("invalid operation: {s}").into()),
        }
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let bad_input = || -> Self::Err { format!("invalid monkey format: '{s}'").into() };
        let (id, job) = s.split_once(": ").ok_or_else(bad_input)?;

        if let Ok(constant) = job.parse() {
            Ok(Self {
                id: id.to_string(),
                output: Some(constant),
                operation: None,
                input1: None,
                input2: None,
            })
        } else {
            let mut parts = job.split_ascii_whitespace();
            let in1 = parts.next().ok_or_else(bad_input)?;
            let op = parts.next().ok_or_else(bad_input)?;
            let in2 = parts.next().ok_or_else(bad_input)?;
            Ok(Self {
                id: id.to_string(),
                output: None,
                operation: Some(op.parse()?),
                input1: Some(Input::Monkey(in1.to_string())),
                input2: Some(Input::Monkey(in2.to_string())),
            })
        }
    }
}

impl Monkey {
    fn has_answer(&self) -> bool {
        self.output.is_some()
    }

    fn answer(&self) -> Datum {
        self.output.unwrap()
    }

    fn receive_answer(&mut self, other: &Monkey) {
        let resolved1 = match &self.input1 {
            Some(Input::Monkey(id)) if id == &other.id => {
                self.input1 = Some(Input::Constant(other.answer()));
                true
            }
            _ => false,
        };
        let resolved2 = match &self.input2 {
            Some(Input::Monkey(id)) if id == &other.id => {
                self.input2 = Some(Input::Constant(other.answer()));
                true
            }
            _ => false,
        };

        if resolved1 || resolved2 {
            match (&self.input1, &self.input2) {
                (Some(Input::Constant(arg1)), Some(Input::Constant(arg2))) => {
                    self.output = Some(match self.operation {
                        Some(Operation::Add) => arg1 + arg2,
                        Some(Operation::Subtract) => arg1 - arg2,
                        Some(Operation::Multiply) => arg1 * arg2,
                        Some(Operation::Divide) => arg1 / arg2,
                        None => panic!("invalid state: received input for a monkey without a job"),
                    })
                }
                _ => {}
            }
        }
    }
}

fn partition(
    monkeys: &[Rc<RefCell<Monkey>>],
) -> (Vec<Rc<RefCell<Monkey>>>, Vec<Rc<RefCell<Monkey>>>) {
    monkeys
        .iter()
        .cloned()
        .partition(|m| m.borrow().has_answer())
}

pub fn part1(input: &str) -> Result<String> {
    let monkeys: Vec<Rc<RefCell<Monkey>>> = input
        .trim()
        .lines()
        .map(|l| {
            let m: Monkey = l.parse()?;
            Ok(Rc::new(RefCell::new(m)))
        })
        .collect::<Result<_>>()?;

    let root: Rc<RefCell<Monkey>> = monkeys
        .iter()
        .find(|m| m.borrow().id == "root")
        .cloned()
        .unwrap();

    let (mut resolved, mut pending): (Vec<_>, Vec<_>) = partition(&monkeys);

    while !root.borrow().has_answer() {
        for monkey in &resolved {
            for listening in &pending {
                let listening = &mut listening.borrow_mut();
                listening.receive_answer(&monkey.borrow());
            }
        }

        (resolved, pending) = partition(&pending);
    }

    let answer = root.borrow().answer();
    Ok(answer.to_string())
}

pub fn part2(_input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day21test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "152")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        todo!("unimplemented");
        // assert_eq!(part2(INPUT).unwrap(), "")
    }
}
