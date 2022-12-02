use std::{collections::HashMap, error::Error};

struct Scores(HashMap<&'static str, i64>);

impl Scores {
    fn part1() -> Self {
        Self(
            [
                ("A X", 3 + 1),
                ("A Y", 6 + 2),
                ("A Z", 0 + 3),
                ("B X", 0 + 1),
                ("B Y", 3 + 2),
                ("B Z", 6 + 3),
                ("C X", 6 + 1),
                ("C Y", 0 + 2),
                ("C Z", 3 + 3),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }

    fn part2() -> Self {
        // X = lose Y = draw Z = win
        Self(
            [
                ("A X", 0 + 3),
                ("A Y", 3 + 1),
                ("A Z", 6 + 2),
                ("B X", 0 + 1),
                ("B Y", 3 + 2),
                ("B Z", 6 + 3),
                ("C X", 0 + 2),
                ("C Y", 3 + 3),
                ("C Z", 6 + 1),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }

    fn get(&self, game: &str) -> Result<i64, Box<dyn Error>> {
        self.0
            .get(game)
            .cloned()
            .ok_or_else(|| format!("invalid game: {}", game).into())
    }

    fn summarize<'a, T: IntoIterator<Item = &'a str>>(
        &self,
        games: T,
    ) -> Result<i64, Box<dyn Error>> {
        games.into_iter().map(|l| self.get(l)).sum()
    }
}

fn summarize_human_readable(input: &str, scores: Scores) -> Result<String, Box<dyn Error>> {
    scores.summarize(input.lines()).map(|s| format!("{}", s))
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    summarize_human_readable(input, Scores::part1())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    summarize_human_readable(input, Scores::part2())
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
