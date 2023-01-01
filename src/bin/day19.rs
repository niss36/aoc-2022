use std::{io, num::ParseIntError, str::FromStr};

use aoc::read_lines;
use regex::Regex;

#[derive(Debug)]
enum Day19Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    ParseBlueprintError,
}

impl From<io::Error> for Day19Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day19Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day19.txt";

fn main() -> Result<(), Day19Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u64,
    ore_robot_ore_cost: u64,
    clay_robot_ore_cost: u64,
    obsidian_robot_ore_cost: u64,
    obsidian_robot_clay_cost: u64,
    geode_robot_ore_cost: u64,
    geode_robot_obsidian_cost: u64,
}

impl FromStr for Blueprint {
    type Err = Day19Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blueprint_regex =
            Regex::new(r"^Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore\. Each clay robot costs ([0-9]+) ore\. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay\. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian\.$")
                .unwrap();

        if let Some(captures) = blueprint_regex.captures(s) {
            Ok(Self {
                id: captures[1].parse()?,
                ore_robot_ore_cost: captures[2].parse()?,
                clay_robot_ore_cost: captures[3].parse()?,
                obsidian_robot_ore_cost: captures[4].parse()?,
                obsidian_robot_clay_cost: captures[5].parse()?,
                geode_robot_ore_cost: captures[6].parse()?,
                geode_robot_obsidian_cost: captures[7].parse()?,
            })
        } else {
            Err(Self::Err::ParseBlueprintError)
        }
    }
}

fn parse_blueprints(input: &Vec<String>) -> Result<Vec<Blueprint>, Day19Error> {
    input.iter().map(|line| line.parse()).collect()
}

#[derive(Debug)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

enum Action {
    DoNothing,
    MakeRobot(RobotType),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,

    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

impl State {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn tick(mut self, blueprint: &Blueprint, action: &Action) -> Self {
        use Action::*;
        use RobotType::*;

        match action {
            DoNothing => {}
            MakeRobot(Ore) => {
                self.ore -= blueprint.ore_robot_ore_cost;
            }
            MakeRobot(Clay) => {
                self.ore -= blueprint.clay_robot_ore_cost;
            }
            MakeRobot(Obsidian) => {
                self.ore -= blueprint.obsidian_robot_ore_cost;
                self.clay -= blueprint.obsidian_robot_clay_cost;
            }
            MakeRobot(Geode) => {
                self.ore -= blueprint.geode_robot_ore_cost;
                self.obsidian -= blueprint.geode_robot_obsidian_cost;
            }
        }

        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        match action {
            DoNothing => {}
            MakeRobot(Ore) => self.ore_robots += 1,
            MakeRobot(Clay) => self.clay_robots += 1,
            MakeRobot(Obsidian) => self.obsidian_robots += 1,
            MakeRobot(Geode) => self.geode_robots += 1,
        }

        self
    }

    fn should_build_more(&self, blueprint: &Blueprint, robot_type: &RobotType) -> bool {
        use RobotType::*;

        match robot_type {
            Ore => {
                self.ore_robots < blueprint.ore_robot_ore_cost
                    || self.ore_robots < blueprint.clay_robot_ore_cost
                    || self.ore_robots < blueprint.obsidian_robot_ore_cost
                    || self.ore_robots < blueprint.geode_robot_ore_cost
            }
            Clay => self.clay_robots < blueprint.obsidian_robot_clay_cost,
            Obsidian => self.obsidian_robots < blueprint.geode_robot_obsidian_cost,
            Geode => true,
        }
    }

    fn time_to_wait(&self, blueprint: &Blueprint, robot_type: &RobotType) -> Option<u64> {
        use RobotType::*;

        match robot_type {
            Ore => {
                if self.ore_robots > 0 {
                    let missing_ore = blueprint.ore_robot_ore_cost.saturating_sub(self.ore);
                    Some(div_ceil(missing_ore, self.ore_robots))
                } else {
                    None
                }
            }
            Clay => {
                if self.ore_robots > 0 {
                    let missing_ore = blueprint.clay_robot_ore_cost.saturating_sub(self.ore);
                    Some(div_ceil(missing_ore, self.ore_robots))
                } else {
                    None
                }
            }
            Obsidian => {
                if self.ore_robots > 0 && self.clay_robots > 0 {
                    let missing_ore = blueprint.obsidian_robot_ore_cost.saturating_sub(self.ore);
                    let missing_clay = blueprint.obsidian_robot_clay_cost.saturating_sub(self.clay);

                    Some(
                        div_ceil(missing_ore, self.ore_robots)
                            .max(div_ceil(missing_clay, self.clay_robots)),
                    )
                } else {
                    None
                }
            }
            Geode => {
                if self.ore_robots > 0 && self.obsidian_robots > 0 {
                    let missing_ore = blueprint.geode_robot_ore_cost.saturating_sub(self.ore);
                    let missing_obsidian = blueprint
                        .geode_robot_obsidian_cost
                        .saturating_sub(self.obsidian);

                    Some(
                        div_ceil(missing_ore, self.ore_robots)
                            .max(div_ceil(missing_obsidian, self.obsidian_robots)),
                    )
                } else {
                    None
                }
            }
        }
    }
}

fn div_ceil(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

fn max_geodes(time_limit: u64, blueprint: &Blueprint) -> u64 {
    let mut result = 0;

    fn aux(
        time_limit: u64,
        blueprint: &Blueprint,
        time_spent: u64,
        state: State,
        result: &mut u64,
    ) {
        use Action::*;
        use RobotType::*;

        assert!(time_spent <= time_limit);

        let mut stuck = true;

        let time_left = time_limit - time_spent;
        if time_left > 0 {
            // g = state.geodes, r = state.geode_robots, n = time_left
            // maximum geodes if we could buy a new geode robot every step:
            // g + r + (r+1) + (r+2) + ... + (r+n-1)
            // = g + n * (r + r + n - 1) / 2
            // = g + n * r + (n * (n - 1)) / 2
            let geode_upper_bound =
                state.geode + time_left * state.geode_robots + (time_left * (time_left - 1)) / 2;

            if geode_upper_bound > *result {
                for robot_type in [Ore, Clay, Obsidian, Geode] {
                    if state.should_build_more(blueprint, &robot_type) {
                        if let Some(time_to_wait) = state.time_to_wait(blueprint, &robot_type) {
                            if time_spent + time_to_wait + 1 <= time_limit {
                                let mut state = state;
                                for _ in 0..time_to_wait {
                                    state = state.tick(blueprint, &DoNothing);
                                }
                                state = state.tick(blueprint, &MakeRobot(robot_type));

                                aux(
                                    time_limit,
                                    blueprint,
                                    time_spent + time_to_wait + 1,
                                    state,
                                    result,
                                );
                                stuck = false;
                            }
                        }
                    }
                }
            }
        }

        if stuck {
            let mut state = state;
            // if no options, advance time until the limit and measure the result
            for _ in time_spent..time_limit {
                state = state.tick(blueprint, &Action::DoNothing);
            }
            *result = (*result).max(state.geode);
        }
    }

    aux(time_limit, blueprint, 0, State::new(), &mut result);

    result
}

fn part1(input: &Vec<String>) -> Result<u64, Day19Error> {
    let blueprints = parse_blueprints(input)?;

    Ok(blueprints
        .into_iter()
        .map(|blueprint| blueprint.id * max_geodes(24, &blueprint))
        .sum())
}

fn part2(input: &Vec<String>) -> Result<u64, Day19Error> {
    let blueprints = parse_blueprints(input)?;

    Ok(blueprints
        .into_iter()
        .take(3)
        .map(|blueprint| max_geodes(32, &blueprint))
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    const EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_parse_blueprint() {
        let blueprint: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse().unwrap();
        let expected = Blueprint {
            id: 1,
            ore_robot_ore_cost: 4,
            clay_robot_ore_cost: 2,
            obsidian_robot_ore_cost: 3,
            obsidian_robot_clay_cost: 14,
            geode_robot_ore_cost: 2,
            geode_robot_obsidian_cost: 7,
        };

        assert_eq!(blueprint, expected);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 33);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 3472);
    }
}
