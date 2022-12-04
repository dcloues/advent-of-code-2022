use std::{error::Error, ops::RangeInclusive};

fn parse_range(r: &str) -> Result<RangeInclusive<i64>, Box<dyn Error>> {
    let parts = r.split_once('-').ok_or(format!("not a range: {}", r))?;
    Ok(parts.0.parse()?..=parts.1.parse()?)
}

fn parse_line(l: &str) -> Result<(RangeInclusive<i64>, RangeInclusive<i64>), Box<dyn Error>> {
    let parts = l.split_once(',').ok_or(format!("not a pair: {}", l))?;
    Ok((parse_range(parts.0)?, parse_range(parts.1)?))
}

fn intersects(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    (r1.contains(r2.start()) && r1.contains(r2.end()))
        || (r2.contains(r1.start()) && r2.contains(r1.end()))
}

fn overlaps(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

pub fn check<F>(input: &str, f: F) -> Result<String, Box<dyn Error>>
where
    F: Fn(&RangeInclusive<i64>, &RangeInclusive<i64>) -> bool,
{
    input
        .lines()
        .try_fold(0, |acc, l| {
            parse_line(l).and_then(|(r1, r2)| Ok(acc + f(&r1, &r2) as i64))
        })
        .map(|count| count.to_string())
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    check(input, intersects)
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    check(input, overlaps)
}

#[cfg(test)]
mod test {
    use super::overlaps;
    use super::part1;
    use super::part2;

    const INPUT: &str = include_str!("tests/day4test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "2");
    }

    #[test]
    fn test_overlaps() {
        assert!(overlaps(&(46..=95), &(68..=88)));
        assert!(overlaps(&(41..=96), &(95..=97)));
        assert!(overlaps(&(10..=10), &(9..=28)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "4");
    }
}
