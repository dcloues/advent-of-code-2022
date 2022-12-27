use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Datum = i64;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

type ID = String;

#[derive(PartialEq, Debug)]
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
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
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
                        Some(Operation::Sub) => arg1 - arg2,
                        Some(Operation::Mul) => arg1 * arg2,
                        Some(Operation::Div) => arg1 / arg2,
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

fn parse_input(
    input: &str,
) -> Result<(
    Vec<Rc<RefCell<Monkey>>>,
    HashMap<String, Rc<RefCell<Monkey>>>,
)> {
    let monkeys: Vec<Rc<RefCell<Monkey>>> = input
        .trim()
        .lines()
        .map(|l| {
            let m: Monkey = l.parse()?;
            Ok(Rc::new(RefCell::new(m)))
        })
        .collect::<Result<_>>()?;

    let by_id = monkeys
        .iter()
        .map(|m| (m.borrow().id.clone(), m.clone()))
        .collect();
    Ok((monkeys, by_id))
}

pub fn part1(input: &str) -> Result<String> {
    let (monkeys, by_id) = parse_input(input)?;
    let (mut resolved, mut pending): (Vec<_>, Vec<_>) = partition(&monkeys);

    let root = by_id["root"].clone();

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

pub fn part2(input: &str) -> Result<String> {
    let (monkeys, by_id) = parse_input(input)?;

    let root = by_id["root"].clone();
    let (input1, input2) = {
        let root = root.borrow();
        let in1 = match &root.input1 {
            Some(Input::Monkey(m)) => by_id[m].clone(),
            _ => panic!("invalid root input"),
        };

        let in2 = match &root.input2 {
            Some(Input::Monkey(m)) => by_id[m].clone(),
            _ => panic!("invalid root input"),
        };
        (in1, in2)
    };

    let (mut resolved, mut pending): (Vec<_>, Vec<_>) = partition(&monkeys);
    loop {
        let mut resolved_any = false;
        for monkey in &resolved {
            // leave the human input unresolved
            if monkey.borrow().id != "humn" {
                for listening in &pending {
                    let listening = &mut listening.borrow_mut();
                    listening.receive_answer(&monkey.borrow());
                    resolved_any = resolved_any || listening.has_answer();
                }
            }
        }

        if resolved_any {
            (resolved, pending) = partition(&pending);
        } else {
            break;
        }
    }

    let (value, pending) = {
        if input1.borrow().has_answer() {
            (input1.borrow().answer(), input2)
        } else {
            (input2.borrow().answer(), input1)
        }
    };

    let human_value = find_human_value(value, pending, &by_id);

    Ok(human_value.to_string())
}

fn find_human_value(
    output: Datum,
    monkey: Rc<RefCell<Monkey>>,
    monkeys: &HashMap<String, Rc<RefCell<Monkey>>>,
) -> Datum {
    if monkey.borrow().id == "humn" {
        output
    } else {
        let monkey = monkey.borrow();
        match (&monkey.operation, &monkey.input1, &monkey.input2) {
            (Some(Operation::Add), Some(Input::Monkey(arg1)), Some(Input::Constant(arg2))) => {
                find_human_value(output - arg2, monkeys[arg1].clone(), monkeys)
            }
            (Some(Operation::Add), Some(Input::Constant(arg1)), Some(Input::Monkey(arg2))) => {
                find_human_value(output - arg1, monkeys[arg2].clone(), monkeys)
            }
            (Some(Operation::Sub), Some(Input::Monkey(arg1)), Some(Input::Constant(arg2))) => {
                // output = arg1 - arg2
                // output + arg2 = arg1
                find_human_value(output + arg2, monkeys[arg1].clone(), monkeys)
            }
            (Some(Operation::Sub), Some(Input::Constant(arg1)), Some(Input::Monkey(arg2))) => {
                // output = arg1 - arg2
                // output + arg2 = arg1
                // arg2 = arg1 - output
                find_human_value(arg1 - output, monkeys[arg2].clone(), monkeys)
            }
            (Some(Operation::Mul), Some(Input::Monkey(arg1)), Some(Input::Constant(arg2))) => {
                find_human_value(output / arg2, monkeys[arg1].clone(), monkeys)
            }
            (Some(Operation::Mul), Some(Input::Constant(arg1)), Some(Input::Monkey(arg2))) => {
                find_human_value(output / arg1, monkeys[arg2].clone(), monkeys)
            }
            (Some(Operation::Div), Some(Input::Monkey(arg1)), Some(Input::Constant(arg2))) => {
                // output = arg1 / arg2
                // arg1 = output * arg2
                find_human_value(output * arg2, monkeys[arg1].clone(), monkeys)
            }
            (Some(Operation::Div), Some(Input::Constant(arg1)), Some(Input::Monkey(arg2))) => {
                // output = arg1 / arg2
                // 1 / output = arg2 / arg1
                // arg1 / output = arg2
                find_human_value(arg1 / output, monkeys[arg2].clone(), monkeys)
            }
            (op, arg1, arg2) => {
                let id = &monkey.id;
                panic!("Monkey {id}: unhandled operation={op:?} output={output} (arg1={arg1:?}, arg2={arg2:?})")
            }
        }
    }
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
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "301")
    }
}
