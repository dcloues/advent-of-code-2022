use std::borrow::Borrow;
#[allow(unused)]
use std::{error::Error, num::ParseIntError, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct Blueprint {
    id: i32,
    ore: RobotRecipe,
    clay: RobotRecipe,
    obsidian: RobotRecipe,
    geode: RobotRecipe,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    time: i32,
    max_time: i32,

    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    final_geodes: i32,

    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,

    built: Option<Resource>,

    previous: Option<Box<State>>,
}

const MAX_TIME: i32 = 24;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(PartialEq, Eq, Debug)]
struct RobotRecipe {
    produces: Resource,
    cost: Vec<(Resource, i32)>,
}

impl Blueprint {
    fn calculate_quality_level(&self) -> i32 {
        self.id
            * self
                .find_best_outcome(&State::default())
                .unwrap()
                .final_geodes
    }
    fn find_best_outcome(&self, start: &State) -> Option<State> {
        if start.time >= MAX_TIME {
            return Some(start.clone());
        }

        self.next_states(&start)
            .iter()
            .map(|next| self.find_best_outcome(next))
            .flatten()
            .max_by(|s1, s2| s1.final_geodes.cmp(&s2.final_geodes))
    }

    fn next_states(&self, state: &State) -> Vec<State> {
        let all_recipes = [&self.geode, &self.obsidian, &self.clay, &self.ore];
        all_recipes
            .iter()
            .map(|r| {
                if r.produces == Resource::Geode {
                    return state.next_state_with(r);
                }
                // do we have enough of this recipe's output to satisfy
                // all possibly future building? if so, don't try for more.
                // println!(
                //     "checking production requirements for {r:?}, current stock is {}",
                //     *state.get_resource(r.produces)
                // );
                let biggest_consumer = all_recipes
                    .iter()
                    .flat_map(|r| r.cost.iter())
                    .filter_map(|(rsc, cost)| (rsc == &r.produces).then_some(cost))
                    .max()
                    .unwrap();

                if state.time_left() * biggest_consumer > *state.get_resource(r.produces) {
                    // println!("  -> need more {r:?}");
                    state.next_state_with(r)
                } else {
                    // println!("  -> NO more {r:?}");
                    None
                }
            })
            .flatten()
            .collect()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            time: 1,
            max_time: MAX_TIME,
            ore: 1,
            clay: Default::default(),
            obsidian: Default::default(),
            geodes: Default::default(),
            final_geodes: Default::default(),
            ore_robots: 1,
            clay_robots: Default::default(),
            obsidian_robots: Default::default(),
            geode_robots: Default::default(),
            previous: None,
            built: None,
        }
    }
}

impl State {
    fn time_left(&self) -> i32 {
        self.max_time - self.time
    }

    fn ticks_to_build(current: i32, robots: i32, cost: i32) -> i32 {
        if current >= cost {
            1
        } else {
            let mut ticks = (cost - current) / robots;
            let remainder = (cost - current) % robots;
            if remainder > 0 {
                ticks += 1;
            }
            ticks + 1
        }
    }

    fn next_state_with(&self, recipe: &RobotRecipe) -> Option<State> {
        let mut time_to_build = 0;
        for (resource, cost) in &recipe.cost {
            if self.get_robots(*resource) == &0 {
                return None;
            }

            let current = self.get_resource(*resource);
            let robots = self.get_robots(*resource);

            time_to_build = time_to_build.max(Self::ticks_to_build(*current, *robots, *cost));

            // let needed = cost - current;
            // time_to_build = 1 + (needed / robots);
            // if time_to_build % robots != 0 {
            //     time_to_build += 1;
            // }
        }

        // time_to_build += 1;

        let mut state = self.step(time_to_build)?;
        for (resource, cost) in &recipe.cost {
            let prev = *state.get_resource(*resource);
            *state.get_resource_mut(*resource) -= cost;
            let new = *state.get_resource(*resource);
            debug_assert!(*state.get_resource(*resource) >= 0, "decremented resource {resource:?} from {prev} to {new} with cost {cost} and time_to_build {time_to_build}");
        }

        *state.get_robots_mut(recipe.produces) += 1;
        state.built = Some(recipe.produces);

        if let Resource::Geode = recipe.produces {
            state.final_geodes += state.max_time - state.time;
        }

        Some(state)
    }

    fn step(&self, time_units: i32) -> Option<Self> {
        if self.time + time_units > self.max_time {
            None
        } else {
            Some(Self {
                time: self.time + time_units,
                ore: self.ore + self.ore_robots * time_units,
                clay: self.clay + self.clay_robots * time_units,
                obsidian: self.obsidian + self.obsidian_robots * time_units,
                geodes: self.geodes + self.geode_robots * time_units,
                previous: Some(Box::new(self.clone())),
                built: None,
                ..*self
            })
        }
    }

    fn get_resource(&self, resource: Resource) -> &i32 {
        match resource {
            Resource::Ore => &self.ore,
            Resource::Clay => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode => &self.geodes,
        }
    }

    fn get_resource_mut(&mut self, resource: Resource) -> &mut i32 {
        match resource {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geodes,
        }
    }

    fn get_robots(&self, resource: Resource) -> &i32 {
        match resource {
            Resource::Ore => &self.ore_robots,
            Resource::Clay => &self.clay_robots,
            Resource::Obsidian => &self.obsidian_robots,
            Resource::Geode => &self.geode_robots,
        }
    }

    fn get_robots_mut(&mut self, resource: Resource) -> &mut i32 {
        match resource {
            Resource::Ore => &mut self.ore_robots,
            Resource::Clay => &mut self.clay_robots,
            Resource::Obsidian => &mut self.obsidian_robots,
            Resource::Geode => &mut self.geode_robots,
        }
    }
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
            ore: RobotRecipe {
                produces: Resource::Ore,
                cost: vec![(Resource::Ore, nth(5, "ore_robot_ore_cost")?)],
            },
            clay: RobotRecipe {
                produces: Resource::Clay,
                cost: vec![(Resource::Ore, nth(5, "clay_robot_ore_cost")?)],
            },
            obsidian: RobotRecipe {
                produces: Resource::Obsidian,
                cost: vec![
                    (Resource::Ore, nth(5, "obsidian_robot_ore_cost")?),
                    (Resource::Clay, nth(2, "obsidian_robot_clay_cost")?),
                ],
            },
            geode: RobotRecipe {
                produces: Resource::Geode,
                cost: vec![
                    (Resource::Ore, nth(5, "geode_robot_ore_cost")?),
                    (Resource::Obsidian, nth(2, "geode_robot_obsidian_cost")?),
                ],
            },
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Blueprint>> {
    input.trim().lines().map(Blueprint::from_str).collect()
}

pub fn part1(input: &str) -> Result<String> {
    let blueprints = parse_input(input)?;
    Ok(blueprints
        .iter()
        .map(|bp| bp.calculate_quality_level())
        .sum::<i32>()
        .to_string())
}

pub fn part2(input: &str) -> Result<String> {
    todo!("unimplemented")
}

fn debug_print_state(state: &State) {
    if let Some(previous) = &state.previous {
        debug_print_state(previous);
    }
    println!("== Minute {} ==", state.time);
    for resource in [
        Resource::Ore,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Geode,
    ] {
        let mut count = *state.get_robots(resource);
        let total = *state.get_resource(resource);
        if total > 0 {
            if state.built == Some(resource) {
                count = count - 1;
            }
            println!("{count} {resource:?} robot(s) collects {count}; total {total}");
        }
    }
    if let Some(built) = state.built {
        println!(
            "The new {built:?} robot is ready; you now have {} of them",
            state.get_robots(built),
        );
    }
    println!();
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
                    ore: RobotRecipe {
                        produces: Resource::Ore,
                        cost: vec![(Resource::Ore, 4)]
                    },
                    clay: RobotRecipe {
                        produces: Resource::Clay,
                        cost: vec![(Resource::Ore, 2)]
                    },
                    obsidian: RobotRecipe {
                        produces: Resource::Obsidian,
                        cost: vec![(Resource::Ore, 3), (Resource::Clay, 14)]
                    },
                    geode: RobotRecipe {
                        produces: Resource::Geode,
                        cost: vec![(Resource::Ore, 2), (Resource::Obsidian, 7)]
                    },
                },
                Blueprint {
                    id: 2,
                    ore: RobotRecipe {
                        produces: Resource::Ore,
                        cost: vec![(Resource::Ore, 2)]
                    },
                    clay: RobotRecipe {
                        produces: Resource::Clay,
                        cost: vec![(Resource::Ore, 3)]
                    },
                    obsidian: RobotRecipe {
                        produces: Resource::Obsidian,
                        cost: vec![(Resource::Ore, 3), (Resource::Clay, 8)]
                    },
                    geode: RobotRecipe {
                        produces: Resource::Geode,
                        cost: vec![(Resource::Ore, 3), (Resource::Obsidian, 12)]
                    },
                },
            ]
        )
    }

    #[test]
    fn test_ticks_to_build() {
        assert_eq!(State::ticks_to_build(2, 1, 2), 1);
        assert_eq!(State::ticks_to_build(2, 1, 3), 2);
        assert_eq!(State::ticks_to_build(2, 2, 5), 3);
    }

    #[test]
    fn test_next_state_for_1() {
        let bps = parse_input(INPUT).unwrap();
        let bp = &bps[0];
        let state = State::default();
        let next = state.next_state_with(&bp.ore).unwrap();
        assert_eq!(
            next,
            State {
                time: 5,
                ore_robots: 2,
                previous: Some(Box::new(state.clone())),
                built: Some(Resource::Ore),
                ..state
            }
        )
    }

    #[test]
    fn test_next_state_for_2() {
        let bps = parse_input(INPUT).unwrap();
        let bp = &bps[0];
        let state = State::default();
        let next = state.next_state_with(&bp.clay).unwrap();

        // We finish t=1 with 1 ore robot and 1 ore
        // We finish t=2 with 1 ore robot and 1+1=2 ore
        // We start t=3 and spend 2 ore to build a clay robot
        // We finish t=3 with 2 ore robots and 1+1+1-2 = 1 ore
        assert_eq!(
            next,
            State {
                time: 3,
                clay_robots: 1,
                previous: Some(Box::new(state.clone())),
                built: Some(Resource::Clay),
                ..state
            }
        )
    }

    #[test]
    fn test_next_state_round() {
        let bps = parse_input(INPUT).unwrap();
        let bp = &bps[0];
        let state = State {
            ore_robots: 2,
            ore: 1,
            ..State::default()
        };

        // ore robot costs 4 ore
        // We finish t=1 with 2 ore robots and 1 ore.
        // We need 3 more ore.
        // We finish t=2 with 1+2 = 3 ore
        // We finish t=3 with 1+2+2 = 5 ore
        // We start building at t=4
        // We finish t=4 with 1+2+2+2-4 = 3 ore and 3 ore robots
        assert_eq!(
            state.next_state_with(&bp.ore).unwrap(),
            State {
                time: 4,
                ore_robots: 3,
                ore: 3,
                previous: Some(Box::new(state.clone())),
                built: Some(Resource::Ore),
                ..state
            }
        )
    }

    #[test]
    fn test_state_iteration() {
        let bps = parse_input(INPUT).unwrap();

        let init: State = State {
            ore_robots: 1,
            ..State::default()
        };

        let best0 = bps[0].find_best_outcome(&init).unwrap();
        debug_print_state(&best0);
        // println!("best0: {best0:?}");
        assert_eq!(best0.geodes, 9);

        let best1 = bps[1].find_best_outcome(&init).unwrap();
        println!("best0: {best1:?}");
        assert_eq!(best1.geodes, 12);
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
