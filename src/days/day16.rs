use core::hash::Hash;
use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    str::FromStr,
};

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

#[derive(PartialEq, Eq, Clone)]
struct State<T> {
    cost: i64,
    node: T,
}
#[derive(Debug)]
struct Edge<T> {
    node: T,
    cost: i64,
}

impl<T> Ord for State<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for State<T>
where
    T: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path<T>(edges: &HashMap<T, Vec<Edge<T>>>, start: T, goal: T) -> Option<i64>
where
    T: Clone + Eq + Ord + Hash,
{
    let mut dist: HashMap<T, i64> = edges.keys().cloned().map(|k| (k, i64::MAX)).collect();
    let mut heap = BinaryHeap::new();
    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }

        if cost > dist[&node] {
            continue;
        }

        if let Some(edges) = edges.get(&node) {
            for edge in edges {
                let next = State {
                    cost: cost + edge.cost,
                    node: edge.node.clone(),
                };
                if next.cost < dist[&next.node] {
                    heap.push(next.clone());
                    dist.insert(next.node.clone(), next.cost);
                }
            }
        }
    }

    None
}

pub fn part1(input: &str) -> Result<String> {
    let caves: Caves = input.parse()?;

    let start: ID = ['A', 'A'];

    let summarize_ids: Vec<ID> = caves
        .valves
        .values()
        .filter_map(|v| {
            if v.id == start || v.flow_rate > 0 {
                Some(v.id)
            } else {
                None
            }
        })
        .collect();

    let all_edges: HashMap<ID, Vec<Edge<ID>>> = caves
        .valves
        .iter()
        .map(|(id, v)| {
            (
                *id,
                v.neighbors
                    .iter()
                    .map(|&node| Edge { cost: 1, node })
                    .collect(),
            )
        })
        .collect();

    let mut summarized: HashMap<ID, Vec<Edge<ID>>> = HashMap::new();
    for src in &summarize_ids {
        let edges = summarize_ids
            .iter()
            .filter_map(|dst| {
                if src == dst {
                    None
                } else if let Some(cached) = summarized.get(dst) {
                    Some(Edge {
                        node: *dst,
                        cost: cached.iter().find(|e| e.node == *src).unwrap().cost,
                    })
                } else {
                    Some(Edge {
                        node: *dst,
                        cost: shortest_path(&all_edges, *src, *dst).unwrap(),
                    })
                }
            })
            .collect();
        summarized.insert(src.clone(), edges);
    }

    println!("condensed edges to: {summarized:#?}");

    todo!("unimplemented")
}

#[allow(unused_variables)]
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
