use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn part1(_input: &str) -> Result<String> {
    todo!("unimplemented")
}

pub fn part2(_input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day21test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "152")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        todo!("unimplemented");
        // assert_eq!(part2(INPUT).unwrap(), "")
    }
}
