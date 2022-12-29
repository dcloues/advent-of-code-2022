use std::{
    collections::HashMap,
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    rows: Vec<Vec<Tile>>,
}

struct State {
    grid: Grid,
    position: (i32, i32), // x, y
    facing: Direction,
    net: Option<Net>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum CubeFace {
    Top,
    North,
    South,
    East,
    West,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
enum Switcheroo {
    RotateRight,
    RotateLeft,
    Rotate180,
}

impl Switcheroo {
    fn invert(self) -> Self {
        match self {
            Switcheroo::RotateRight => Self::RotateLeft,
            Switcheroo::RotateLeft => Self::RotateRight,
            Switcheroo::Rotate180 => Self::Rotate180,
        }
    }
}

impl<T: Clone> Cubed<T> {
    fn select(&self, face: CubeFace) -> &T {
        match face {
            CubeFace::Top => &self.top,
            CubeFace::North => &self.north,
            CubeFace::South => &self.south,
            CubeFace::East => &self.east,
            CubeFace::West => &self.west,
            CubeFace::Bottom => &self.bottom,
        }
    }
}

type NetOrigin = (usize, usize);
type Edge = (CubeFace, Direction, CubeFace, Switcheroo);

#[derive(Clone)]
struct Net {
    origins: Cubed<NetOrigin>,
    edges: HashMap<(CubeFace, Direction), (CubeFace, Switcheroo)>,
    dim: i32,
}

impl Net {
    fn new(dim: i32, origins: Cubed<NetOrigin>, edges: &[Edge]) -> Self {
        let edges = edges
            .iter()
            .cloned()
            .flat_map(|(from, dir, to, switch)| {
                [
                    ((from, dir), (to, switch)),
                    (
                        (to, dir.transform(switch.invert())),
                        (from, switch.invert()),
                    ),
                ]
            })
            .collect();
        println!("new New with edges:");
        for edge in &edges {
            println!("  {edge:?}");
        }
        Self {
            dim,
            origins,
            edges,
        }
    }
}

#[derive(Clone)]
struct Cubed<T: Clone> {
    top: T,
    north: T,
    south: T,
    east: T,
    west: T,
    bottom: T,
}

fn split_grid(grid: &Grid) -> Net {
    let dim = if grid.width() > 50 { 50 } else { 4 };
    // top north south east west bottom
    let nets = [
        // from the example
        Net::new(
            dim,
            Cubed {
                top: (2, 0),
                south: (2, 1),
                west: (1, 1),
                north: (0, 1),
                bottom: (2, 2),
                east: (3, 2),
            },
            &[
                (
                    CubeFace::Top,
                    Direction::Left,
                    CubeFace::West,
                    Switcheroo::RotateLeft,
                ),
                (
                    CubeFace::Top,
                    Direction::Up,
                    CubeFace::North,
                    Switcheroo::Rotate180,
                ),
                (
                    CubeFace::Top,
                    Direction::Right,
                    CubeFace::East,
                    Switcheroo::Rotate180,
                ),
                (
                    CubeFace::South,
                    Direction::Right,
                    CubeFace::East,
                    Switcheroo::RotateRight,
                ),
                (
                    CubeFace::Bottom,
                    Direction::Down,
                    CubeFace::North,
                    Switcheroo::Rotate180,
                ),
                (
                    CubeFace::Bottom,
                    Direction::Left,
                    CubeFace::West,
                    Switcheroo::RotateRight,
                ),
                (
                    CubeFace::North,
                    Direction::Left,
                    CubeFace::East,
                    Switcheroo::RotateRight,
                ),
            ],
        ),
        // from my AOC input
        // Net {
        //     edges: vec![],
        //     origins: Cubed {
        //         top: (1, 0),
        //         east: (2, 0),
        //         south: (1, 1),
        //         bottom: (1, 2),
        //         west: (0, 2),
        //         north: (0, 3),
        //     },
        // },
    ];

    nets.iter()
        .find(|n| n.matches(dim as usize, grid))
        .unwrap()
        .clone()
}

impl Net {
    fn step(&self, position: (i32, i32), direction: Direction) -> ((i32, i32), Direction) {
        // what face does it belong to?
        let (face, (local_x, local_y)) = self
            .faces()
            .iter()
            .find_map(|(face, origin)| {
                if position.0 / self.dim == origin.0 as i32
                    && position.1 / self.dim == origin.1 as i32
                {
                    Some((
                        *face,
                        (
                            position.0 - (self.dim * origin.0 as i32),
                            position.1 - (self.dim * origin.1 as i32),
                        ),
                    ))
                } else {
                    None
                }
            })
            .unwrap();

        let max_dim = self.dim - 1;
        if let Some((to_face, transform)) = self.edges.get(&(face, direction)) {
            let target_position = match (direction, transform) {
                (Direction::Up, Switcheroo::RotateRight) => (0, local_x),
                (Direction::Up, Switcheroo::RotateLeft) => (max_dim, max_dim - local_x),
                (Direction::Up, Switcheroo::Rotate180) => (max_dim - local_x, 0),
                (Direction::Down, Switcheroo::RotateRight) => (max_dim, local_x),
                (Direction::Down, Switcheroo::RotateLeft) => (0, max_dim - local_x),
                (Direction::Down, Switcheroo::Rotate180) => (max_dim - local_x, max_dim),
                (Direction::Left, Switcheroo::RotateRight) => (max_dim, max_dim - local_y),
                (Direction::Left, Switcheroo::RotateLeft) => (0, local_y),
                (Direction::Left, Switcheroo::Rotate180) => (max_dim, max_dim - local_y),
                (Direction::Right, Switcheroo::RotateRight) => (max_dim - local_y, 0),
                (Direction::Right, Switcheroo::RotateLeft) => (local_y, max_dim),
                (Direction::Right, Switcheroo::Rotate180) => (max_dim, max_dim - local_y),
            };

            println!("moved from {face:?} ({local_x},{local_y}) to {to_face:?} {target_position:?} via {transform:?}");
            let origin = self.origins.select(*to_face);
            let target_position = (
                target_position.0 + self.dim * origin.0 as i32,
                target_position.1 + self.dim * origin.1 as i32,
            );

            (target_position, direction.transform(*transform))
        } else {
            panic!("stepped into the void from face={face:?} direction={direction:?} position={position:?}");
        }
    }

    fn faces(&self) -> Vec<(CubeFace, &NetOrigin)> {
        vec![
            (CubeFace::Top, &self.origins.top),
            (CubeFace::Bottom, &self.origins.bottom),
            (CubeFace::South, &self.origins.south),
            (CubeFace::North, &self.origins.north),
            (CubeFace::East, &self.origins.east),
            (CubeFace::West, &self.origins.west),
        ]
    }

    fn matches(&self, dim: usize, grid: &Grid) -> bool {
        self.valid_face(CubeFace::Top, dim, grid)
            && self.valid_face(CubeFace::Bottom, dim, grid)
            && self.valid_face(CubeFace::North, dim, grid)
            && self.valid_face(CubeFace::South, dim, grid)
            && self.valid_face(CubeFace::East, dim, grid)
            && self.valid_face(CubeFace::West, dim, grid)
    }

    fn valid_face(&self, face: CubeFace, dim: usize, grid: &Grid) -> bool {
        let origin = match face {
            CubeFace::Top => self.origins.top,
            CubeFace::North => self.origins.north,
            CubeFace::South => self.origins.south,
            CubeFace::East => self.origins.east,
            CubeFace::West => self.origins.west,
            CubeFace::Bottom => self.origins.bottom,
        };

        grid.get(((dim * origin.0) as i32, (dim * origin.1) as i32)) != Tile::Void
    }
}

impl Direction {
    fn invert(self) -> Self {
        self.turn_right().turn_right()
    }

    fn transform(self, transform: Switcheroo) -> Self {
        match transform {
            Switcheroo::RotateRight => self.turn_right(),
            Switcheroo::RotateLeft => self.turn_left(),
            Switcheroo::Rotate180 => self.invert(),
        }
    }

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
            net: None,
        }
    }

    fn password(&self) -> String {
        println!(
            "Final location x={} y={} facing={:?}",
            self.position.0 + 1,
            self.position.1 + 1,
            self.facing,
        );
        let row = (1 + self.position.1) * 1000;
        let col = (1 + self.position.0) * 4;
        let facing_value = match self.facing {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        };
        (row + col + facing_value).to_string()
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

            let mut new_direction = self.facing;
            let (x, y) = new_position;
            if self.grid.get(new_position) == Tile::Void {
                if let Some(net) = &self.net {
                    (new_position, new_direction) = net.step(self.position, self.facing);
                } else {
                    new_position = match self.facing {
                        Direction::Up => self.grid.column(x).last().unwrap().0,
                        Direction::Down => self.grid.column(x).next().unwrap().0,
                        Direction::Left => self.grid.row(y).last().unwrap().0,
                        Direction::Right => self.grid.row(y).next().unwrap().0,
                    }
                }
            }

            match self.grid.get(new_position) {
                Tile::Open => {
                    self.position = new_position;
                    self.facing = new_direction;
                }
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
    Ok(state.password())
}

pub fn part2(input: &str) -> Result<String> {
    let (grid, moves) = parse_input(input)?;

    let net = split_grid(&grid);
    let mut state = State::new(grid);
    state.net = Some(net);
    state.apply_moves(&moves);
    Ok(state.password())
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
    fn test_fold() {
        let (grid, _) = parse_input(INPUT).unwrap();
        split_grid(&grid);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "5031");
    }
}
