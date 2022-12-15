use std::{
    error::Error,
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    rows: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

pub struct LocationRange {
    start: Location,
    end: Location,
}

impl<T: PartialEq + Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rows: vec![vec![T::default(); width]; height],
        }
    }

    #[allow(dead_code)]
    pub fn from_vec(v: Vec<Vec<T>>) -> Self {
        Self {
            width: v[0].len(),
            height: v.len(),
            rows: v,
        }
    }

    #[allow(dead_code)]
    pub fn find(&self, item: &T) -> Option<Location> {
        self.rows.iter().enumerate().find_map(|(row, cols)| {
            cols.iter()
                .position(|t: &T| t == item)
                .map(|col| Location { row, col })
        })
    }

    pub fn get(&self, loc: Location) -> Option<&T> {
        self.rows.get(loc.row).and_then(|r| r.get(loc.col))
    }

    pub fn set(&mut self, loc: Location, item: T) {
        self.rows[loc.row][loc.col] = item
    }

    pub fn locations<'a>(&'a self) -> Box<dyn Iterator<Item = Location> + 'a> {
        Box::new((0..self.height).flat_map(|r| (0..self.width).map(move |c| Location::new(r, c))))
    }

    #[allow(dead_code)]
    pub fn neighbors(&self, loc: Location) -> Vec<Location> {
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

    #[allow(dead_code)]
    pub fn up(&self, loc: Location) -> Option<Location> {
        (loc.row > 0).then(|| Location::new(loc.row - 1, loc.col))
    }

    #[allow(dead_code)]
    pub fn down(&self, loc: Location) -> Option<Location> {
        (loc.row < self.height - 1).then(|| Location::new(loc.row + 1, loc.col))
    }

    #[allow(dead_code)]
    pub fn left(&self, loc: Location) -> Option<Location> {
        (loc.col > 0).then(|| Location::new(loc.row, loc.col - 1))
    }

    #[allow(dead_code)]
    pub fn right(&self, loc: Location) -> Option<Location> {
        (loc.col < self.width - 1).then(|| Location::new(loc.row, loc.col + 1))
    }

    #[allow(dead_code)]
    pub fn rows(&self) -> &Vec<Vec<T>> {
        &self.rows
    }

    pub fn expand(&mut self, rows: usize, columns: usize) {
        for row in &mut self.rows {
            row.resize(columns, T::default())
        }
        self.rows.resize(rows, vec![T::default(); columns]);
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for item in row {
                f.write_str(&item.to_string())?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Location {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn up(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    #[allow(dead_code)]
    pub fn down(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    pub fn right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    pub fn left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    pub fn to(&self, other: &Location) -> Option<LocationRange> {
        if self.row == other.row || self.col == other.col {
            Some(LocationRange {
                start: *self.min(other),
                end: *self.max(other),
            })
        } else {
            None
        }
    }
}

impl Iterator for LocationRange {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            let current = self.start;
            if self.start.row == self.end.row {
                self.start.col += 1;
            } else {
                self.start.row += 1;
            }
            Some(current)
        } else {
            None
        }
    }
}

impl FromStr for Location {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((c, r)) => Ok(Self::new(r.parse()?, c.parse()?)),
            None => Err(format!("malformed location '{s}'").into()),
        }
    }
}
