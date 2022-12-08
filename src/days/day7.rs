use std::{collections::HashMap, error::Error};

fn parse_dir_sizes(input: &str) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let mut sizes: HashMap<String, i64> = HashMap::new();

    let mut current_dir: Vec<&str> = vec![];
    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.as_slice() {
            &["$", "ls"] => {} // ignore,
            &["dir", _] => {}  // ignore
            &["$", "cd", "/"] => current_dir.truncate(0),
            &["$", "cd", ".."] => {
                if current_dir.pop().is_none() {
                    return Err(format!("invalid cd .. from {:?}", current_dir).into());
                }
            }
            &["$", "cd", child] => current_dir.push(child),
            &[size, _] => {
                let size: i64 = size.parse()?;
                let mut path: String = "/".to_string();
                *sizes.entry(path.clone()).or_insert(0) += size;
                for elem in &current_dir {
                    path.push('/');
                    path.push_str(elem);
                    *sizes.entry(path.clone()).or_insert(0) += size;
                }
            }
            _ => panic!("unhandled input line {}", line),
        }
    }

    Ok(sizes)
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let sizes = parse_dir_sizes(input)?;
    Ok(sizes
        .values()
        .filter(|size| **size <= 100000)
        .sum::<i64>()
        .to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let sizes = parse_dir_sizes(input)?;
    let capacity = 70000000;
    let required = 30000000;
    let to_free = required - (capacity - sizes["/"]);

    let deleted = sizes.values().filter(|size| **size > to_free).min();

    deleted
        .map(|s| s.to_string())
        .ok_or_else(|| "couldn't find dir to delete".into())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day7test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "95437")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "24933642")
    }
}
