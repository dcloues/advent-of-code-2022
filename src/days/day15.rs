use std::{collections::HashSet, error::Error, ops::RangeInclusive, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Signal {
    sensor: Point,
    signal: Point,
}

type Span = (i32, i32);

impl Point {
    fn taxicab_distance(&self, other: &Point) -> i32 {
        self.dx(other) + self.dy(other)
    }

    fn dx(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
    }

    fn dy(&self, other: &Point) -> i32 {
        (self.y - other.y).abs()
    }
}

impl FromStr for Signal {
    type Err = Box<dyn Error>;

    fn from_str(mut s: &str) -> std::result::Result<Self, Self::Err> {
        if !s.starts_with("Sensor at ") {
            return Err(format!("bad input: {s}").into());
        }
        s = &s["Sensor at ".len()..];
        let points = s
            .split_once(": closest beacon is at ")
            .ok_or_else(|| -> Box<dyn Error> { format!("bad input: {s}").into() })?;

        Ok(Self {
            sensor: points.0.parse()?,
            signal: points.1.parse()?,
        })
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //x=-399822, y=4026621
        match s.split(&[' ', ',', '=']).collect::<Vec<_>>().as_slice() {
            &["x", x, "", "y", y] => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            }),
            _ => Err(format!("invaid Point: {s}").into()),
        }
    }
}

impl Signal {
    fn rowspan(&self, y: i32) -> Option<Span> {
        let dy = (self.sensor.y - y).abs();
        if dy <= self.radius() {
            // We use up dy's worth of units getting
            // from our sensor to the target row,
            // which leaves us with radius-dy units
            // left to go in either direction.
            // It is valid for halfwidth to be 0.
            let halfwidth = self.radius() - dy;
            Some((self.sensor.x - halfwidth, self.sensor.x + halfwidth))
        } else {
            None
        }
    }

    fn radius(&self) -> i32 {
        self.sensor.taxicab_distance(&self.signal)
    }
}

fn get_exclusions(signals: &Vec<Signal>, row: i32) -> Vec<Span> {
    let mut spans: Vec<Span> = signals.iter().filter_map(|s| s.rowspan(row)).collect();
    spans.sort();
    spans.iter().cloned().fold(vec![], |mut acc, span| {
        match acc.last_mut() {
            Some(prev) if prev.1 >= span.0 => {
                prev.1 = prev.1.max(span.1);
            }
            _ => acc.push(span),
        }
        acc
    })
}

fn count_exclusions_in_row(signals: &Vec<Signal>, row: i32) -> i32 {
    let sum: i32 = get_exclusions(signals, row)
        .iter()
        .map(|(i, j)| (j - i) + 1)
        .sum();

    let signals_on_row = signals
        .iter()
        .filter_map(|sig| (sig.signal.y == row).then_some(sig.signal.x))
        .collect::<HashSet<_>>()
        .len() as i32;

    return sum - signals_on_row;
}

fn find_distress_beacon(
    signals: &Vec<Signal>,
    xs: RangeInclusive<i32>,
    ys: RangeInclusive<i32>,
) -> Option<Point> {
    for y in ys {
        let exclusions: Vec<Span> = get_exclusions(&signals, y)
            .iter()
            .cloned()
            .filter(|x| !(&x.1 < xs.start() || &x.0 > xs.end()))
            .collect();

        match exclusions.as_slice() {
            &[ex] if ex.0 > *xs.start() => return Some(Point { x: *xs.start(), y }),
            &[ex] if ex.1 < *xs.end() => return Some(Point { x: *xs.end(), y }),
            &[ex, _] => return Some(Point { x: ex.1 + 1, y }),
            _ => {}
        };
    }
    None
}

fn tuning_frequency(point: Point) -> i64 {
    (point.x as i64 * 4000000) + point.y as i64
}

fn parse_input(input: &str) -> Result<Vec<Signal>> {
    input.lines().map(Signal::from_str).collect::<Result<_>>()
}

pub fn part1(input: &str) -> Result<String> {
    let signals: Vec<Signal> = parse_input(input)?;
    Ok(count_exclusions_in_row(&signals, 2000000).to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let signals: Vec<Signal> = parse_input(input)?;
    let beacon = find_distress_beacon(&signals, 0..=4000000, 0..=4000000)
        .ok_or("could not locate beacon".to_string())?;

    Ok(tuning_frequency(beacon).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day15test.txt");

    #[test]
    fn test_get_exclusions() {
        let signals: Vec<Signal> = parse_input(INPUT).unwrap();
        assert_eq!(get_exclusions(&signals, 10), vec![(-2, 24)]);
    }

    #[test]
    fn test_part1() {
        let signals: Vec<Signal> = parse_input(INPUT).unwrap();
        assert_eq!(count_exclusions_in_row(&signals, 10), 26);
    }

    #[test]
    fn test_part2() {
        let signals: Vec<Signal> = parse_input(INPUT).unwrap();
        assert_eq!(
            find_distress_beacon(&signals, 0..=20, 0..=20),
            Some(Point { x: 14, y: 11 })
        );
    }

    #[test]
    fn test_tuning() {
        assert_eq!(tuning_frequency(Point { x: 14, y: 11 }), 56000011);
    }
}
