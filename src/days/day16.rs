use std::{collections::HashMap, error::Error, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type ID = [char; 2];

#[derive(Debug, PartialEq, Eq)]
struct Valve {
    id: ID,
    flow_rate: i32,
    neighbors: Vec<ID>,
}

#[derive(Debug)]
struct Caves {
    valves: HashMap<ID, Valve>,
}

fn parse_id(s: &str) -> Result<ID> {
    if s.chars().count() == 2 {
        let mut c = s.chars();
        Ok([c.next().unwrap(), c.next().unwrap()])
    } else {
        Err(format!("invalid id '{s}'").into())
    }
}

impl FromStr for Valve {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.split(' ');
        tokens.next();

        let id = parse_id(tokens.next().ok_or_else(|| "missing ID".to_string())?)?;

        tokens.next();
        tokens.next();

        let flow_rate = tokens.next().ok_or_else(|| "missing rate".to_string())?;
        let flow_rate: i32 = flow_rate
            .split_once('=')
            .ok_or_else(|| "invalid rate".to_string())?
            .1
            .trim_end_matches(';')
            .parse()
            .map_err(|e| -> Box<dyn Error> { format!("invalid rate {e}").into() })?;

        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();

        let neighbors: Vec<ID> = tokens
            .map(|s| parse_id(s.trim_end_matches(',')))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            id,
            flow_rate,
            neighbors,
        })
    }
}

impl FromStr for Caves {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let valves = s
            .lines()
            .map(|l| {
                let v: Valve = l.parse()?;
                Ok((v.id, v))
            })
            .collect::<Result<_>>()?;

        Ok(Self { valves })
    }
}

pub fn part1(input: &str) -> Result<String> {
    todo!("unimplemented")
}

pub fn part2(input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day16test.txt");

    #[test]
    fn test_parse() {
        assert_eq!(
            Valve::from_str("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB").unwrap(),
            Valve {
                id: ['A', 'A'],
                flow_rate: 0,
                neighbors: vec![['D', 'D'], ['I', 'I'], ['B', 'B']],
            }
        );

        assert_eq!(
            Valve::from_str("Valve HH has flow rate=22; tunnel leads to valve GG").unwrap(),
            Valve {
                id: ['H', 'H'],
                flow_rate: 22,
                neighbors: vec![['G', 'G']],
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
