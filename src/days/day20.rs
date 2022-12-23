use std::error::Error;

pub fn part1(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day7test.txt");

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "3")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        todo!("unimplemented");
    }
}
