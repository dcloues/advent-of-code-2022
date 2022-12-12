use std::{cell::RefCell, error::Error, num::ParseIntError, rc::Rc, str::FromStr};

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(Debug)]
enum Operand {
    Constant(i64),
    Old,
}

#[derive(Debug)]
struct Monkey {
    id: i64,
    items: Vec<i64>,
    op: Operation,
    test_divisor: i64,
    true_target: i64,
    false_target: i64,
    total_inspections: i64,
}

impl FromStr for Operand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "old" {
            Operand::Old
        } else {
            Operand::Constant(s.parse()?)
        })
    }
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err(format!("invalid Operation: {s}").into());
        }

        let op1 = parts[0].parse()?;
        let op2 = parts[2].parse()?;
        match parts[1] {
            "+" => Ok(Operation::Add(op1, op2)),
            "*" => Ok(Operation::Mul(op1, op2)),
            _ => Err(format!("invalid operator {} in {}", parts[1], s).into()),
        }
    }
}

struct Throw {
    item: i64,
    monkey: i64,
}

impl Operation {
    fn apply(&self, item: i64) -> i64 {
        match self {
            Operation::Add(op1, op2) => op1.resolve(item) + op2.resolve(item),
            Operation::Mul(op1, op2) => op1.resolve(item) * op2.resolve(item),
        }
    }
}

impl Operand {
    fn resolve(&self, item: i64) -> i64 {
        match self {
            Operand::Constant(i) => *i,
            Operand::Old => item,
        }
    }
}

impl Monkey {
    fn inspect_and_throw_item(&mut self, item: i64) -> Throw {
        let new_item: i64 = self.op.apply(item) / 3;
        self.total_inspections += 1;

        Throw {
            item: new_item,
            monkey: if new_item % self.test_divisor == 0 {
                self.true_target
            } else {
                self.false_target
            },
        }
    }

    fn inspect_and_throw(&mut self) -> Vec<Throw> {
        let items: Vec<i64> = self.items.drain(..).collect();
        items
            .iter()
            .map(|i| self.inspect_and_throw_item(*i))
            .collect()
    }

    fn parse_id(s: &str) -> Result<i64, Box<dyn Error>> {
        if s.starts_with("Monkey ") && s.ends_with(':') {
            s["Monkey ".len()..s.len() - 1]
                .parse::<i64>()
                .map_err(|e| e.into())
        } else {
            Err(format!("Not a Monkey: {s}").into())
        }
    }

    fn parse_items(s: &str) -> Result<Vec<i64>, Box<dyn Error>> {
        Self::parse_line(s, "Starting items:", |s| {
            s.split(", ")
                .map(|i| i.parse())
                .collect::<Result<_, _>>()
                .map_err(|e: ParseIntError| e.into())
        })
    }

    fn parse_line<T, F>(line: &str, prefix: &str, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce(&str) -> Result<T, Box<dyn Error>>,
    {
        if line.starts_with(prefix) {
            f(&line[prefix.len()..line.len()].trim())
        } else {
            Err(format!("expected line starting with {prefix}, got {line}").into())
        }
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").map(str::trim).collect();
        if lines.len() != 6 {
            return Err(format!("Invalid monkey - need 6 lines, got {}", lines.len()).into());
        }

        Ok(Self {
            total_inspections: 0,
            id: Self::parse_id(lines[0])?,
            items: Self::parse_items(lines[1])?,
            op: Self::parse_line(lines[2], "Operation: new =", Operation::from_str)?,
            test_divisor: Self::parse_line(lines[3], "Test: divisible by", |s| {
                s.parse().map_err(|e: ParseIntError| e.into())
            })?,
            true_target: Self::parse_line(lines[4], "If true: throw to monkey", |s| {
                s.parse().map_err(|e: ParseIntError| e.into())
            })?,
            false_target: Self::parse_line(lines[5], "If false: throw to monkey", |s| {
                s.parse().map_err(|e: ParseIntError| e.into())
            })?,
        })
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let monkeys: Vec<Rc<RefCell<Monkey>>> = input
        .trim()
        .split("\n\n")
        .map(|m| -> Result<Rc<RefCell<Monkey>>, Box<dyn Error>> {
            Ok(Rc::new(RefCell::new(Monkey::from_str(m)?)))
        })
        .collect::<Result<Vec<Rc<RefCell<_>>>, _>>()?;

    for _ in 0..20 {
        for monkey in &monkeys {
            for throw in &monkey.borrow_mut().inspect_and_throw() {
                monkeys[throw.monkey as usize]
                    .borrow_mut()
                    .items
                    .push(throw.item);
            }
        }
    }

    for monkey in &monkeys {
        println!(
            "monkey {} has inspections {}",
            monkey.borrow().id,
            monkey.borrow().total_inspections
        );
    }

    let mut counts: Vec<i64> = monkeys
        .iter()
        .map(|m| m.borrow().total_inspections)
        .collect();

    counts.sort();
    Ok(counts[counts.len() - 2..]
        .iter()
        .product::<i64>()
        .to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day11test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "10605")
    }

    fn test_part2() {
        todo!("unimplemented");
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
