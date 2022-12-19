use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
    num::ParseIntError,
    ops::RangeInclusive,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

impl Point {
    fn neighbors(&self) -> [Point; 6] {
        [
            Point {
                x: self.x + 1,
                ..*self
            },
            Point {
                x: self.x - 1,
                ..*self
            },
            Point {
                y: self.y + 1,
                ..*self
            },
            Point {
                y: self.y - 1,
                ..*self
            },
            Point {
                z: self.z + 1,
                ..*self
            },
            Point {
                z: self.z - 1,
                ..*self
            },
        ]
    }
}

fn parse_input<'a>(input: &'a str) -> Box<dyn Iterator<Item = Result<Point>> + 'a> {
    Box::new(input.lines().map(|l| -> Result<_> {
        let units: [i64; 3] = l
            .split(',')
            .map(|s| -> Result<i64> { s.parse().map_err(|e: ParseIntError| e.into()) })
            .collect::<Result<Vec<_>>>()?
            .as_slice()
            .try_into()?;

        Ok(Point {
            x: units[0],
            y: units[1],
            z: units[2],
        })
    }))
}

#[derive(Default)]
struct State {
    edges: HashSet<Point>,
    surface: i64,
    reachable_surface: i64,
}

impl State {
    fn is_surrounded(&self, edge: &Point) -> bool {
        edge.neighbors().iter().all(|n| self.edges.contains(n))
    }

    fn run(mut self, input: &str) -> Result<Self> {
        for block in parse_input(input) {
            let block = block?;
            self.surface += 6;
            self.edges.insert(block);
            for neighbor in block.neighbors() {
                if self.edges.contains(&neighbor) {
                    // subtract one for our face, one for neighbor's face
                    self.surface -= 2;
                    if self.is_surrounded(&neighbor) {
                        self.edges.remove(&neighbor);
                    }
                }
            }
        }

        let mut ranges =
            self.edges
                .iter()
                .fold(((0i64..=0), (0i64..=0), (0i64..=0)), |(x, y, z), edge| {
                    (
                        *x.start().min(&edge.x)..=*x.end().max(&edge.x),
                        *y.start().min(&edge.y)..=*y.end().max(&edge.y),
                        *z.start().min(&edge.z)..=*z.end().max(&edge.z),
                    )
                });

        ranges = (
            ranges.0.start() - 1..=ranges.0.end() + 1,
            ranges.1.start() - 1..=ranges.1.end() + 1,
            ranges.2.start() - 1..=ranges.2.end() + 1,
        );

        self.reachable_surface = self.explore_surface(
            Point {
                x: *ranges.0.start(),
                y: *ranges.1.start(),
                z: *ranges.2.start(),
            },
            &ranges,
        );

        Ok(self)
    }

    fn explore_surface(
        &self,
        pt: Point,
        (rx, ry, rz): &(
            RangeInclusive<i64>,
            RangeInclusive<i64>,
            RangeInclusive<i64>,
        ),
    ) -> i64 {
        let mut reachable = 0;
        let mut visited: HashSet<Point> = HashSet::new();
        let mut next: VecDeque<Point> = VecDeque::new();
        next.push_front(pt);

        while let Some(pt) = next.pop_front() {
            for n in pt.neighbors() {
                if visited.contains(&n)
                    || !rx.contains(&n.x)
                    || !ry.contains(&n.y)
                    || !rz.contains(&n.z)
                {
                    continue;
                }

                if self.edges.contains(&n) {
                    reachable += 1;
                } else {
                    visited.insert(n);
                    next.push_back(n);
                }
            }
        }

        reachable
    }
}

pub fn part1(input: &str) -> Result<String> {
    let state = State::default().run(input)?;

    Ok(state.surface.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let state = State::default().run(input)?;

    Ok(state.reachable_surface.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day18test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "64");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "58")
    }
}
