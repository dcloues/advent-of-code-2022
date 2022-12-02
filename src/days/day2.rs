use std::{collections::HashMap, error::Error};

fn summarize_human_readable(
    input: &str,
    scores: &[(&str, i64)], //&HashMap<&str, i64>,
) -> Result<String, Box<dyn Error>> {
    let scores: HashMap<_, _> = scores.iter().cloned().collect();
    input
        .lines()
        .map(|l| {
            scores
                .get(l)
                .ok_or_else(|| format!("invalid game {}", l).into())
        })
        .sum::<Result<i64, _>>()
        .map(|sum| format!("{}", sum))
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let scores = [
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ];
    summarize_human_readable(input, &scores)
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let scores = [
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ];
    summarize_human_readable(input, &scores)
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const TEST_INPUT: &str = include_str!("tests/day2test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT).unwrap(), "15")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT).unwrap(), "12")
    }
}
