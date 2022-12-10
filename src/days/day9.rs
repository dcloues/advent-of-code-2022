use std::{cmp::Ordering, collections::HashSet, error::Error, str::FromStr};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("invalid direction '{}'", s)),
        }
    }
}

fn parse_line(l: &str) -> Result<(Direction, usize), Box<dyn Error>> {
    match l.split_once(' ') {
        Some((d, n)) => Ok((d.parse()?, n.parse()?)),
        None => Err(format!("invalid input line {}", l).into()),
    }
}

fn iter_line(
    line: Result<(Direction, usize), Box<dyn Error>>,
) -> Box<dyn Iterator<Item = Result<Direction, Box<dyn Error>>>> {
    match line {
        Ok((d, n)) => Box::new(std::iter::from_fn(move || Some(Ok(d))).take(n)),
        Err(e) => Box::new(std::iter::once(Err(e))),
    }
}

fn parse_input<'a>(
    input: &'a str,
) -> Box<dyn Iterator<Item = Result<Direction, Box<dyn Error>>> + 'a> {
    Box::new(input.lines().flat_map(|line| iter_line(parse_line(line))))
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_by(&self, d: Direction) -> Self {
        Self {
            x: self.x
                + match d {
                    Direction::Left => -1,
                    Direction::Right => 1,
                    _ => 0,
                },
            y: self.y
                + match d {
                    Direction::Up => 1,
                    Direction::Down => -1,
                    _ => 0,
                },
        }
    }

    fn follow(&self, head: Point) -> Self {
        if self.distance(head) <= 1 {
            return self.clone();
        }

        Self::new(follow(self.x, head.x), follow(self.y, head.y))
    }

    fn distance(&self, other: Point) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx.max(dy)
    }
}

fn follow(tail: i32, head: i32) -> i32 {
    tail + match tail.cmp(&head) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn traverse(input: &str, rope_length: usize) -> Result<HashSet<Point>, Box<dyn Error>> {
    let mut rope = vec![Point::default(); rope_length];
    let mut seen = HashSet::new();
    seen.insert(rope[0]);

    for cmd in parse_input(input) {
        rope[0] = rope[0].move_by(cmd?);
        for i in 1..rope.len() {
            rope[i] = rope[i].follow(rope[i - 1]);
        }

        seen.insert(*rope.last().unwrap());
    }

    Ok(seen)
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    traverse(input, 2).map(|seen| seen.len().to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    traverse(input, 10).map(|seen| seen.len().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = include_str!("tests/day9test1.txt");
    const INPUT2: &str = include_str!("tests/day9test2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1).unwrap(), "13")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2).unwrap(), "36")
    }
}
