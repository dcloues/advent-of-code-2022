use std::error::Error;

fn type_score(ch: char) -> Result<i64, String> {
    if ch.is_ascii_uppercase() {
        Ok(ch as i64 - 65 + 27)
    } else if ch.is_ascii_lowercase() {
        Ok(ch as i64 - 97 + 1)
    } else {
        Err(format!("invalid package type '{}'", ch))
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let scored_lines: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.chars().map(type_score).collect::<Result<Vec<_>, _>>())
        .collect::<Result<_, _>>()?;
    let s: Result<i64, _> = scored_lines
        .iter()
        .map(|line| {
            let (p1, p2) = line.split_at(line.len() / 2);
            p1.iter()
                .find(|score| p2.contains(score))
                .ok_or("invalid input - no duplicated item".into())
        })
        .sum();
    s.map(|s| s.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented");
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::type_score;

    const INPUT: &str = include_str!("tests/day3test.txt");

    #[test]
    fn test_part1_scores() {
        assert_eq!(type_score('a').unwrap(), 1);
        assert_eq!(type_score('z').unwrap(), 26);
        assert_eq!(type_score('A').unwrap(), 27);
        assert_eq!(type_score('Z').unwrap(), 52);
        assert!(type_score('!').is_err());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "157");
    }
}
