use std::error::Error;

fn find_boundary(input: &str, size: usize) -> Result<String, Box<dyn Error>> {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(size)
        .enumerate()
        .find_map(|(i, chs)| {
            if chs
                .iter()
                .enumerate()
                .all(|(i, c)| !chs[i + 1..].contains(c))
            {
                Some((i + size).to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| format!("invalid input {}", input).into())
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    find_boundary(input, 4)
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    find_boundary(input, 14)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), "7");
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), "5");
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), "6");
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), "10");
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), "19");
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), "23");
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), "23");
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), "29");
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), "26");
    }
}
