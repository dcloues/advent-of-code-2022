use std::{error::Error, num::ParseIntError, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct Blueprint {
    id: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

impl FromStr for Blueprint {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(&[' ', ':']);
        let mut nth = |n, field: &str| -> Result<i32> {
            let value = parts.nth(n).ok_or_else(|| -> Box<dyn Error> {
                format!("Invalid blueprint (could not parse {field}): {s}").into()
            })?;

            value.parse().map_err(|e: ParseIntError| {
                format!("Could not parse {field} value '{value}': {e}").into()
            })
        };

        Ok(Self {
            id: nth(1, "id")?,
            ore_robot_ore_cost: nth(5, "ore_robot_ore_cost")?,
            clay_robot_ore_cost: nth(5, "clay_robot_ore_cost")?,
            obsidian_robot_ore_cost: nth(5, "obsidian_robot_ore_cost")?,
            obsidian_robot_clay_cost: nth(2, "obsidian_robot_clay_cost")?,
            geode_robot_ore_cost: nth(5, "geode_robot_ore_cost")?,
            geode_robot_obsidian_cost: nth(2, "geode_robot_obsidian_cost")?,
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Blueprint>> {
    input.trim().lines().map(Blueprint::from_str).collect()
}

pub fn part1(input: &str) -> Result<String> {
    todo!("unimplemented")
}

pub fn part2(input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day19test.txt");

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input(INPUT).unwrap(),
            [
                Blueprint {
                    id: 1,
                    ore_robot_ore_cost: 4,
                    clay_robot_ore_cost: 2,
                    obsidian_robot_ore_cost: 3,
                    obsidian_robot_clay_cost: 14,
                    geode_robot_ore_cost: 2,
                    geode_robot_obsidian_cost: 7,
                },
                Blueprint {
                    id: 2,
                    ore_robot_ore_cost: 2,
                    clay_robot_ore_cost: 3,
                    obsidian_robot_ore_cost: 3,
                    obsidian_robot_clay_cost: 8,
                    geode_robot_ore_cost: 3,
                    geode_robot_obsidian_cost: 12,
                }
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "33")
    }

    fn test_part2() {
        todo!("unimplemented");
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
