#[allow(unused)]
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

#[derive(Clone, Default)]
struct State {
    time: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    final_geodes: i32,

    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,

    skip_ore: bool,
    skip_clay: bool,
    skip_obsidian: bool,
    skip_geode: bool,
}

const MAX_TIME: i32 = 24;

enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct RobotRecipe {
    produces: Resource,
    cost: Vec<(Resource, i32)>,
}

impl Blueprint {
    fn find_best_outcome(&self, start: &State) -> State {
        if start.time > MAX_TIME {
            return start.clone();
        }

        self.next_states(&start)
            .iter()
            .map(|next| self.find_best_outcome(next))
            .max_by(|s1, s2| s1.final_geodes.cmp(&s2.final_geodes))
            .unwrap()
    }

    fn max_required_ore_stockpile(&self, time: i32) -> i32 {
        (MAX_TIME - time)
            * [
                self.ore_robot_ore_cost,
                self.clay_robot_ore_cost,
                self.obsidian_robot_ore_cost,
                self.geode_robot_ore_cost,
            ]
            .iter()
            .max()
            .unwrap()
            .clone()
    }

    fn max_required_clay_stockpile(&self, time: i32) -> i32 {
        (MAX_TIME - time) * self.obsidian_robot_clay_cost
    }

    fn max_required_obsidian_stockpile(&self, time: i32) -> i32 {
        (MAX_TIME - time) * self.geode_robot_obsidian_cost
    }

    fn next_states(&self, state: &State) -> Vec<State> {
        let mut states = Vec::new();

        // try to fast forward until we can build an ore robot
        // if state.ore <= self.max_required_ore_stockpile(state.time) {
        //     if state.ore >= self.ore_robot_ore_cost {
        //         states.push(State {
        //             ore: with_resources.ore - self.ore_robot_ore_cost,
        //             ore_robots: with_resources.ore_robots + 1,
        //             ..state.collect_resources()
        //         })
        //     } else {
        //         let fast_forward = self.ore_robot_ore_cost /
        //     }
        // }

        states
    }

    fn next_states_x(&self, state: &State) -> Vec<State> {
        let mut states = Vec::new();
        let with_resources = state.collect_resources();
        states.push(with_resources.clone());

        // Use the starting state for all construction checks,
        // because construction starts before robots finish collecting
        // resources.
        let can_build_ore = state.ore >= self.ore_robot_ore_cost
            && state.ore < self.max_required_ore_stockpile(state.time);
        if can_build_ore {
            states.push(State {
                ore: with_resources.ore - self.ore_robot_ore_cost,
                ore_robots: with_resources.ore_robots + 1,
                ..with_resources
            })
        }

        if state.ore >= self.clay_robot_ore_cost
            && state.clay < self.max_required_clay_stockpile(state.time)
        {
            states.push(State {
                ore: with_resources.ore - self.clay_robot_ore_cost,
                clay_robots: with_resources.clay_robots + 1,
                ..with_resources
            })
        }

        // if state.ore >= self.obsidian_robot_ore_cost && state.clay >= self.obsidian_robot_clay_cost
        // {
        //     states.push(State {
        //         ore: with_resources.ore - self.obsidian_robot_ore_cost,
        //         clay: with_resources.clay - self.obsidian_robot_clay_cost,
        //         obsidian_robots: with_resources.obsidian_robots + 1,
        //         ..with_resources
        //     })
        // }

        // if state.ore >= self.geode_robot_ore_cost
        //     && state.obsidian >= self.geode_robot_obsidian_cost
        // {
        //     states.push(State {
        //         ore: with_resources.ore - self.geode_robot_ore_cost,
        //         obsidian: with_resources.obsidian - self.geode_robot_obsidian_cost,
        //         geode_robots: with_resources.geode_robots + 1,
        //         final_geodes: with_resources.final_geodes + (MAX_TIME - state.time),
        //         ..with_resources
        //     })
        // }

        states
    }
}

impl State {
    fn collect_resources(&self) -> Self {
        Self {
            time: self.time + 1,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..*self
        }
    }
}

// fn find_optimal(state: &State, blueprint: &Blueprint) -> (&Blueprint, State) {
//     if state.time == MAX_TIME {
//         return
//     }
// }

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
    let blueprints = parse_input(input)?;

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
    fn test_state_iteration() {
        let bps = parse_input(INPUT).unwrap();

        let init: State = State {
            ore_robots: 1,
            ..State::default()
        };

        assert_eq!(bps[0].find_best_outcome(&init).final_geodes, 9);
        assert_eq!(bps[1].find_best_outcome(&init).final_geodes, 12);
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
