use std::error::Error;

fn type_score(ch: char) -> Result<i64, Box<dyn Error>> {
    println!("type_score {}", ch);
    if ch.is_ascii_uppercase() {
        Ok(ch as i64 - 65 + 27)
    } else if ch.is_ascii_lowercase() {
        Ok(ch as i64 - 97 + 1)
    } else {
        Err(format!("invalid package type '{}'", ch).into())
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    input
        .lines()
        .try_fold(0, |acc, line| -> Result<i64, _> {
            let scored: Vec<i64> = line.chars().map(type_score).collect::<Result<_, _>>()?;
            let (p1, p2) = scored.split_at(scored.len() / 2);
            p1.iter()
                .find(|s| p2.contains(s))
                .map(|s| acc + s)
                .ok_or("invalid input - no duplicated item".into())
        })
        .map(|s| s.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .chunks(3)
        .try_fold(0, |acc, group| {
            group[0]
                .chars()
                .find(|c| group.iter().all(|line| line.contains(*c)))
                .ok_or("invalid input".into())
                .and_then(type_score)
                .map(|score| acc + score)
        })
        .map(|score| score.to_string())
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;
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

    #[test]
    fn test_part2() {
        println!("part 2");
        assert_eq!(part2(INPUT).unwrap(), "70");
    }
}
