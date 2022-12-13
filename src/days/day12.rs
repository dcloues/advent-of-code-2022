use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    rows: Vec<Vec<T>>,
}

#[derive(Debug, PartialEq, Eq)]
struct SearchState<T> {
    node: T,
    cost: i64,
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl<T: PartialEq> Grid<T> {
    fn find(&self, item: &T) -> Option<Location> {
        self.rows.iter().enumerate().find_map(|(row, cols)| {
            cols.iter()
                .position(|t: &T| t == item)
                .map(|col| Location { row, col })
        })
    }

    fn get(&self, loc: Location) -> Option<&T> {
        self.rows.get(loc.row).and_then(|r| r.get(loc.col))
    }

    fn set(&mut self, loc: Location, item: T) {
        self.rows[loc.row][loc.col] = item
    }

    fn locations<'a>(&'a self) -> Box<dyn Iterator<Item = Location> + 'a> {
        Box::new((0..self.height).flat_map(|r| (0..self.width).map(move |c| Location::new(r, c))))
    }

    fn neighbors(&self, loc: Location) -> Vec<Location> {
        [
            self.up(loc),
            self.down(loc),
            self.left(loc),
            self.right(loc),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<Location>>()
    }

    fn up(&self, loc: Location) -> Option<Location> {
        (loc.row > 0).then(|| Location::new(loc.row - 1, loc.col))
    }

    fn down(&self, loc: Location) -> Option<Location> {
        (loc.row < self.height - 1).then(|| Location::new(loc.row + 1, loc.col))
    }

    fn left(&self, loc: Location) -> Option<Location> {
        (loc.col > 0).then(|| Location::new(loc.row, loc.col - 1))
    }

    fn right(&self, loc: Location) -> Option<Location> {
        (loc.col < self.width - 1).then(|| Location::new(loc.row, loc.col + 1))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row).then(self.col.cmp(&other.col))
    }
}

impl<T> FromStr for Grid<T>
where
    T: From<char>,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<T>> = s
            .trim()
            .split("\n")
            .map(|r| r.chars().map(T::from).collect())
            .collect();

        if !rows.iter().all(|r| r.len() == rows[0].len()) {
            Err("all rows must have the same length".into())
        } else {
            Ok(Self {
                width: rows[0].len(),
                height: rows.len(),
                rows: rows,
            })
        }
    }
}

impl<T: Ord> Ord for SearchState<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then(self.node.cmp(&other.node))
    }
}

impl<T: Ord> PartialOrd for SearchState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let grid: Grid<u32> = input.parse()?;
    let start = grid
        .find(&'S'.into())
        .ok_or::<Box<dyn Error>>("could not find starting point".into())?;

    search(grid, &[start])
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let grid: Grid<u32> = input.parse()?;
    let starts: Vec<Location> = grid
        .locations()
        .filter(|l| *grid.get(*l).unwrap() == 'a'.into())
        .collect();
    search(grid, &starts)
}

fn search(mut grid: Grid<u32>, starts: &[Location]) -> Result<String, Box<dyn Error>> {
    let end = grid
        .find(&'E'.into())
        .ok_or::<Box<dyn Error>>("could not find destination".into())?;

    let mut visited: HashMap<Location, i64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    for &start in starts {
        grid.set(start, 'a'.into());
        visited.insert(start, 0);
        heap.push(SearchState {
            node: start,
            cost: 0,
        });
    }
    grid.set(end, 'z'.into());

    while let Some(current) = heap.pop() {
        if current.node == end {
            return Ok(current.cost.to_string());
        }

        if let Some(best_cost) = visited.get(&current.node) {
            if *best_cost < current.cost {
                continue;
            }
        }

        let location = current.node;
        let elevation = grid.get(current.node).unwrap();

        grid.neighbors(location)
            .iter()
            .filter(|l| *grid.get(**l).unwrap() <= elevation + 1)
            .for_each(|next_loc| {
                let next = SearchState {
                    cost: current.cost + 1,
                    node: *next_loc,
                };
                let best_cost = visited.entry(next.node).or_insert(i64::MAX);

                if next.cost < *best_cost {
                    *best_cost = next.cost;
                    heap.push(next);
                }
            });
    }

    Err("couldn't get there from here".into())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day12test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "31")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "29")
    }
}
