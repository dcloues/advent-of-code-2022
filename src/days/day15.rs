use std::error::Error;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day15test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "26")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
