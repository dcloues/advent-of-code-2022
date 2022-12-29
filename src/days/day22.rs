use std::{
    error::Error,
    fmt::{Display, Write},
    rc::Rc,
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

struct Grid {
    rows: Vec<Vec<Tile>>,
}

struct State {
    grid: Grid,
    position: (i32, i32), // x, y
    facing: Direction,
}

struct State2 {
    cube: Cube,
    face: CubeFace,
    position: (i32, i32), // x, y
    facing: Direction,
}

#[derive(Clone)]
enum CubeFace {
    Top,
    North,
    South,
    East,
    West,
    Bottom,
}

impl CubeFace {
    fn neighbor(self, d: Direction) -> Self {
        match (self, d) {
            (CubeFace::Top, Direction::Up) => Self::North,
            (CubeFace::Top, Direction::Down) => Self::South,
            (CubeFace::Top, Direction::Left) => Self::West,
            (CubeFace::Top, Direction::Right) => Self::East,
            (CubeFace::North, Direction::Up) => Self::Top,
            (CubeFace::North, Direction::Down) => Self::Bottom,
            (CubeFace::North, Direction::Left) => todo!(),
            (CubeFace::North, Direction::Right) => todo!(),
            (CubeFace::South, Direction::Up) => todo!(),
            (CubeFace::South, Direction::Down) => todo!(),
            (CubeFace::South, Direction::Left) => todo!(),
            (CubeFace::South, Direction::Right) => Self::East,
            (CubeFace::East, Direction::Up) => todo!(),
            (CubeFace::East, Direction::Down) => todo!(),
            (CubeFace::East, Direction::Left) => todo!(),
            (CubeFace::East, Direction::Right) => todo!(),
            (CubeFace::West, Direction::Up) => todo!(),
            (CubeFace::West, Direction::Down) => todo!(),
            (CubeFace::West, Direction::Left) => todo!(),
            (CubeFace::West, Direction::Right) => todo!(),
            (CubeFace::Bottom, Direction::Up) => todo!(),
            (CubeFace::Bottom, Direction::Down) => todo!(),
            (CubeFace::Bottom, Direction::Left) => todo!(),
            (CubeFace::Bottom, Direction::Right) => todo!(),
        }
    }
}

struct Cube {
    faces: Cubed<Face>,
    edges: Vec<Edge>,
}

struct Face {
    face: CubeFace,
    origin: (i32, i32),
    grid: Grid,
}

#[derive(Clone)]
enum Switcheroo {
    RotateRight,
    RotateLeft,
    Mirror,
}

impl<T> Cubed<T> {
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
type Edge = (CubeFace, CubeFace, Switcheroo);

struct Net {
    origins: Cubed<NetOrigin>,
    edges: Vec<Edge>,
}

struct Cubed<T> {
    top: T,
    north: T,
    south: T,
    east: T,
    west: T,
    bottom: T,
}

fn split_grid(grid: Grid) -> Cube {
    let dim = if grid.width() > 50 { 50 } else { 4 };
    // top north south east west bottom
    let nets = [
        // from the example
        Net {
            origins: Cubed {
                top: (2, 0),
                south: (2, 1),
                west: (1, 1),
                north: (0, 1),
                bottom: (2, 2),
                east: (3, 2),
            },
            edges: vec![
                (CubeFace::Top, CubeFace::West, Switcheroo::RotateLeft),
                (CubeFace::Top, CubeFace::North, Switcheroo::Mirror),
                (CubeFace::Top, CubeFace::East, Switcheroo::Mirror),
                (CubeFace::South, CubeFace::East, Switcheroo::RotateRight),
                (CubeFace::Bottom, CubeFace::North, Switcheroo::Mirror),
                (CubeFace::Bottom, CubeFace::West, Switcheroo::RotateRight),
                (CubeFace::North, CubeFace::East, Switcheroo::RotateRight),
            ],
        },
        // from my AOC input
        Net {
            edges: vec![],
            origins: Cubed {
                top: (1, 0),
                east: (2, 0),
                south: (1, 1),
                bottom: (1, 2),
                west: (0, 2),
                north: (0, 3),
            },
        },
    ];

    nets.iter().find_map(|s| s.fold(dim, &grid)).unwrap()
}

impl Net {
    fn fold(&self, dim: usize, grid: &Grid) -> Option<Cube> {
        Some(Cube {
            edges: self.edges.clone(),
            faces: Cubed {
                top: self.get_face(CubeFace::Top, dim, grid)?,
                bottom: self.get_face(CubeFace::Bottom, dim, grid)?,
                north: self.get_face(CubeFace::North, dim, grid)?,
                south: self.get_face(CubeFace::South, dim, grid)?,
                east: self.get_face(CubeFace::East, dim, grid)?,
                west: self.get_face(CubeFace::West, dim, grid)?,
            },
        })
    }

    fn get_face(&self, face: CubeFace, dim: usize, grid: &Grid) -> Option<Face> {
        let origin = match face {
            CubeFace::Top => self.origins.top,
            CubeFace::North => self.origins.north,
            CubeFace::South => self.origins.south,
            CubeFace::East => self.origins.east,
            CubeFace::West => self.origins.west,
            CubeFace::Bottom => self.origins.bottom,
        };

        if grid.get(((dim * origin.0) as i32, (dim * origin.1) as i32)) == Tile::Void {
            return None;
        }

        let y_range = dim * origin.1..dim * (1 + origin.1);
        let x_range = dim * origin.0..dim * (1 + origin.0);
        Some(Face {
            face,
            origin: (x_range.start as i32, y_range.start as i32),
            grid: Grid {
                rows: grid.rows[y_range]
                    .iter()
                    .map(|r| r[x_range.clone()].to_vec())
                    .collect(),
            },
        })
    }
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

impl State2 {
    fn new(cube: Cube) -> Self {
        Self {
            cube: cube,
            face: CubeFace::Top,
            position: (0, 0),
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

            let face: &Face = self.cube.faces.select(self.face);

            let (x, y) = new_position;
            if face.grid.get(new_position) == Tile::Void {

                // new_position = match self.facing {
                //     Direction::Up => self.grid.column(x).last().unwrap().0,
                //     Direction::Down => self.grid.column(x).next().unwrap().0,
                //     Direction::Left => self.grid.row(y).last().unwrap().0,
                //     Direction::Right => self.grid.row(y).next().unwrap().0,
                // }
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
    fn test_fold() {
        let (grid, _) = parse_input(INPUT).unwrap();
        let cube = split_grid(grid);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        // assert_eq!(part2(INPUT).unwrap(), "")
    }
}
