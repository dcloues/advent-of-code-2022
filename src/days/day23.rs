use std::{
    collections::HashSet,
    collections::VecDeque,
    error::Error,
    fmt::{Display, Formatter, Write},
    ops::Add,
    ops::Sub,
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

struct TryMove {
    mv: Point,
    tests: &'static [Point; 3],
}

const NORTH: Point = Point::new(0, -1);
const NORTHEAST: Point = Point::new(1, -1);
const EAST: Point = Point::new(1, 0);
const SOUTHEAST: Point = Point::new(1, 1);
const SOUTH: Point = Point::new(0, 1);
const SOUTHWEST: Point = Point::new(-1, 1);
const WEST: Point = Point::new(-1, 0);
const NORTHWEST: Point = Point::new(-1, -1);

const TRY_MOVE_NORTH: TryMove = TryMove {
    mv: NORTH,
    tests: &[NORTH, NORTHEAST, NORTHWEST],
};
const TRY_MOVE_SOUTH: TryMove = TryMove {
    mv: SOUTH,
    tests: &[SOUTH, SOUTHEAST, SOUTHWEST],
};
const TRY_MOVE_WEST: TryMove = TryMove {
    mv: WEST,
    tests: &[WEST, NORTHWEST, SOUTHWEST],
};
const TRY_MOVE_EAST: TryMove = TryMove {
    mv: EAST,
    tests: &[EAST, NORTHEAST, SOUTHEAST],
};

const TRY_MOVES: [TryMove; 4] = [TRY_MOVE_NORTH, TRY_MOVE_SOUTH, TRY_MOVE_WEST, TRY_MOVE_EAST];

impl Point {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn neighbors(self) -> [Point; 8] {
        [
            self + NORTH,
            self + NORTHEAST,
            self + EAST,
            self + SOUTHEAST,
            self + SOUTH,
            self + SOUTHWEST,
            self + WEST,
            self + NORTHWEST,
        ]
    }
}

impl<T> From<(T, T)> for Point
where
    T: Into<i64>,
{
    fn from(t: (T, T)) -> Self {
        Self {
            x: t.0.into(),
            y: t.1.into(),
        }
    }
}

impl TryMove {
    fn try_move(&self, elf: &Point, elves: &HashSet<Point>) -> Option<Point> {
        if self.tests.iter().all(|test| !elves.contains(&(test + elf))) {
            Some(self.mv + *elf)
        } else {
            None
        }
    }

    fn find_move<'a>(
        moves: impl IntoIterator<Item = &'a TryMove>,
        elf: &Point,
        elves: &HashSet<Point>,
    ) -> Option<Point> {
        moves.into_iter().find_map(|m| m.try_move(elf, elves))
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        &self + &other
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Default)]
struct Field {
    elves: HashSet<Point>,
}

impl Field {
    fn step_n(&mut self, n: i64) {
        let mut moves = VecDeque::from(TRY_MOVES);

        for _ in 1..=n {
            self.step(&moves);
            moves.rotate_left(1);
        }
    }

    fn has_neighbors(&self, elf: Point) -> bool {
        elf.neighbors().iter().any(|n| self.elves.contains(n))
    }

    fn step(&mut self, moves: &VecDeque<TryMove>) {
        self.elves = self
            .elves
            .iter()
            .map(|elf| self.move_elf(*elf, &moves))
            .collect();
    }

    fn move_elf(&self, elf: Point, moves: &VecDeque<TryMove>) -> Point {
        if !self.has_neighbors(elf) {
            return elf;
        }

        match TryMove::find_move(moves, &elf, &self.elves) {
            Some(plan) if !self.is_collision(&elf, plan, moves) => plan,
            _ => elf,
        }
    }

    #[must_use]
    fn is_collision(&self, elf: &Point, wants_move: Point, moves: &VecDeque<TryMove>) -> bool {
        let probes = [
            wants_move + NORTH,
            wants_move + SOUTH,
            wants_move + EAST,
            wants_move + WEST,
        ];

        probes
            .iter()
            .filter(|p| *p != elf && self.elves.contains(p))
            .any(|probe| {
                self.has_neighbors(*probe)
                    && TryMove::find_move(moves, probe, &self.elves) == Some(wants_move)
            })
    }

    fn find_bounds(&self) -> (Point, Point) {
        let (x_min, y_min, x_max, y_max) = self.elves.iter().fold(
            (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
            |(x_min, y_min, x_max, y_max), pt| {
                (
                    x_min.min(pt.x),
                    y_min.min(pt.y),
                    x_max.max(pt.x),
                    y_max.max(pt.y),
                )
            },
        );

        (Point { x: x_min, y: y_min }, Point { x: x_max, y: y_max })
    }

    fn find_free_area(&self) -> i64 {
        let (min, max) = self.find_bounds();
        let diff = max - min;
        ((diff.x + 1) * (diff.y + 1)).abs() - self.elves.len() as i64
    }
}

impl FromStr for Field {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Field> {
        let elves: HashSet<Point> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    (c == '#').then(|| Point {
                        x: x as i64,
                        y: y as i64,
                    })
                })
            })
            .collect();

        Ok(Field { elves })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bounds = self.find_bounds();
        for y in bounds.0.y..=bounds.1.y {
            for x in bounds.0.x..=bounds.1.x {
                if self.elves.contains(&Point { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        std::fmt::Result::Ok(())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

pub fn part1(input: &str) -> Result<String> {
    let mut field: Field = input.parse()?;
    field.step_n(10);

    Ok(field.find_free_area().to_string())
}

pub fn part2(_input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day23test.txt");

    #[test]
    fn test_find_move_available() {
        let moves = VecDeque::from(TRY_MOVES);
        let elf = Point { x: 1, y: 1 };
        let elves: HashSet<Point> = [elf].into();
        assert_eq!(
            TryMove::find_move(&moves, &elf, &elves),
            Some(Point { x: 1, y: 0 })
        );
    }

    #[test]
    fn test_find_move_second() {
        let moves = VecDeque::from(TRY_MOVES);
        let elf = Point { x: 1, y: 1 };
        let neighbor = Point { x: 0, y: 0 };
        let elves: HashSet<Point> = [elf, neighbor].into();
        assert_eq!(
            TryMove::find_move(&moves, &elf, &elves),
            Some(Point { x: 1, y: 2 })
        );
    }

    #[test]
    fn test_find_move_bug() {
        let moves = VecDeque::from(TRY_MOVES);
        let elf = Point { x: 3, y: 3 };
        let elves: HashSet<Point> = [
            (2, 0).into(),
            (3, 3).into(),
            (2, 2).into(),
            (3, 0).into(),
            (2, 4).into(),
        ]
        .into();

        assert_eq!(
            TryMove::find_move(&moves, &elf, &elves),
            Some(Point { x: 4, y: 3 })
        );
    }

    #[test]
    fn test_bounds() {
        let field: Field = INPUT.parse().unwrap();
        assert_eq!(
            field.find_bounds(),
            (Point { x: 0, y: 0 }, Point { x: 6, y: 6 })
        );

        assert_eq!(field.find_free_area(), 27);
    }

    #[test]
    fn test_elf_neighbors() {
        let elf = (0, 0).into();

        for mv in [
            NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST,
        ] {
            let field = Field {
                elves: [elf, elf + mv].into(),
            };
            assert_eq!(field.has_neighbors(elf), true);
        }
    }

    #[test]
    #[ignore]
    fn test_part1_large() {
        let input = include_str!("tests/day23test_large.txt");
        assert_eq!(part1(input).unwrap(), "812");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "110")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
