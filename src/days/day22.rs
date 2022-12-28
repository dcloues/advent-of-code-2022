use std::{
    error::Error,
    fmt::{Display, Write},
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Void,
    Open,
    Wall,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Move {
    Step(usize),
    Turn(Turn),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self, t: Turn) -> Self {
        match t {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct Grid {
    rows: Vec<Vec<Tile>>,
}

struct State {
    grid: Grid,
    position: (i32, i32), // x, y
    facing: Direction,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Void => ' ',
            Tile::Open => '.',
            Tile::Wall => '#',
        })
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Grid {
            rows: s
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            ' ' => Ok(Tile::Void),
                            '.' => Ok(Tile::Open),
                            '#' => Ok(Tile::Wall),
                            _ => Err(format!("bad tile {c}").into()),
                        })
                        .collect::<Result<Vec<Tile>>>()
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Grid {
    fn get(&self, loc: (i32, i32)) -> Tile {
        if loc.0 < 0 || loc.1 < 0 {
            return Tile::Void;
        }

        self.rows
            .get(loc.1 as usize)
            .and_then(|r| r.get(loc.0 as usize))
            .cloned()
            .or_else(|| Some(Tile::Void))
            .unwrap()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn column<'a>(
        &'a self,
        x: i32,
    ) -> Box<dyn DoubleEndedIterator<Item = ((i32, i32), Tile)> + 'a> {
        Box::new((0..self.height()).into_iter().filter_map(move |y| {
            let y = y as i32;
            match self.get((x, y as i32)) {
                Tile::Void => None,
                t => Some(((x, y), t)),
            }
        }))
    }

    fn row<'a>(&'a self, y: i32) -> Box<dyn DoubleEndedIterator<Item = ((i32, i32), Tile)> + 'a> {
        Box::new((0..self.width()).into_iter().filter_map(move |x| {
            let x = x as i32;
            match self.get((x, y)) {
                Tile::Void => None,
                t => Some(((x, y), t)),
            }
        }))
    }
}

impl State {
    fn new(grid: Grid) -> Self {
        let col = grid.rows[0].iter().position(|t| *t == Tile::Open).unwrap() as i32;
        println!(
            "State::new detected starting column {col} in {:?}",
            grid.rows[0]
        );
        Self {
            grid,
            position: (col, 0),
            facing: Direction::Right,
        }
    }

    fn apply_moves(&mut self, moves: &[Move]) {
        for mv in moves {
            self.apply_move(mv);
        }
    }

    fn apply_move(&mut self, mv: &Move) {
        println!(
            "applying move: {mv:?} from {:?} {:?}",
            self.position, self.facing
        );
        match mv {
            Move::Step(n) => self.step(*n),
            Move::Turn(t) => self.facing = self.facing.turn(*t),
        }
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            let mut new_position = match self.facing {
                Direction::Up => (self.position.0, self.position.1 - 1),
                Direction::Down => (self.position.0, self.position.1 + 1),
                Direction::Left => (self.position.0 - 1, self.position.1),
                Direction::Right => (self.position.0 + 1, self.position.1),
            };

            let (x, y) = new_position;
            if self.grid.get(new_position) == Tile::Void {
                new_position = match self.facing {
                    Direction::Up => self.grid.column(x).last().unwrap().0,
                    Direction::Down => self.grid.column(x).next().unwrap().0,
                    Direction::Left => self.grid.row(y).last().unwrap().0,
                    Direction::Right => self.grid.row(y).next().unwrap().0,
                }
            }

            match self.grid.get(new_position) {
                Tile::Open => self.position = new_position,
                Tile::Wall => return,
                Tile::Void => panic!("logic error: wrapped around to void"),
            }
            println!("moved to {new_position:?}");
        }
    }
}

fn parse_input(input: &str) -> Result<(Grid, Vec<Move>)> {
    let (grid, moves) = input
        .trim_end()
        .split_once("\n\n")
        .ok_or("couldn't split raw input".to_string())?;

    let moves: Vec<Move> = moves
        .split_inclusive(&['R', 'L'])
        .flat_map(|m| {
            if m.ends_with(&['R', 'L']) {
                vec![
                    Move::Step(m[0..m.len() - 1].parse().unwrap()),
                    if m.ends_with('R') {
                        Move::Turn(Turn::Right)
                    } else {
                        Move::Turn(Turn::Left)
                    },
                ]
            } else {
                vec![Move::Step(m.parse().unwrap())]
            }
        })
        .collect();

    Ok((grid.parse()?, moves))
}

pub fn part1(input: &str) -> Result<String> {
    let (grid, moves) = parse_input(input)?;

    let mut state = State::new(grid);
    state.apply_moves(&moves);
    println!(
        "Final location x={} y={} facing={:?}",
        state.position.0 + 1,
        state.position.1 + 1,
        state.facing,
    );
    let row = (1 + state.position.1) * 1000;
    let col = (1 + state.position.0) * 4;
    let facing_value = match state.facing {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    };
    Ok((row + col + facing_value).to_string())
}

pub fn part2(_input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day22test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "6032")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        // assert_eq!(part2(INPUT).unwrap(), "")
    }
}
