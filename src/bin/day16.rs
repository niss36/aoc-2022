use std::{
    collections::{HashMap, HashSet},
    io,
    num::ParseIntError,
    str::FromStr,
};

use aoc::read_lines;
use regex::Regex;

#[derive(Debug)]
enum Day16Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    ParseValveError,
    EmptyInput,
    ValveNotFound,
}

impl From<io::Error> for Day16Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day16Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day16.txt";

fn main() -> Result<(), Day16Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve {
    label: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = Day16Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valve_regex =
            Regex::new(r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z]+(?:, [A-Z]+)*)$")
                .unwrap();

        if let Some(captures) = valve_regex.captures(s) {
            let label = captures[1].to_string();
            let flow_rate = captures[2].parse()?;
            let tunnels = captures[3].split(", ").map(|s| s.to_string()).collect();

            Ok(Self {
                label,
                flow_rate,
                tunnels,
            })
        } else {
            Err(Self::Err::ParseValveError)
        }
    }
}

fn parse_valves(input: &Vec<String>) -> Result<HashMap<String, Valve>, Day16Error> {
    input
        .iter()
        .map(|line| line.parse())
        .map(|maybe_valve| maybe_valve.map(|valve: Valve| (valve.label.clone(), valve)))
        .collect()
}

#[derive(Debug, Clone)]
enum Action {
    OpenValve,
    MoveTo(String),
    NoOp,
}

#[derive(Debug, Clone)]
struct VolcanoState<'a> {
    valves: &'a HashMap<String, Valve>,
    current_position: &'a Valve,
    open_valves: HashSet<&'a Valve>,
    pressure_released: u64,
}

impl<'a> VolcanoState<'a> {
    fn new(
        valves: &'a HashMap<String, Valve>,
        starting_position: &String,
    ) -> Result<Self, Day16Error> {
        let current_position = valves
            .get(starting_position)
            .ok_or(Day16Error::EmptyInput)?;

        Ok(Self {
            valves,
            current_position,
            open_valves: HashSet::new(),
            pressure_released: 0,
        })
    }

    fn tick(&mut self, action: Action) -> Result<(), Day16Error> {
        for v in &self.open_valves {
            self.pressure_released += v.flow_rate;
        }

        match action {
            Action::OpenValve => {
                self.open_valves.insert(self.current_position);
                Ok(())
            }
            Action::MoveTo(label) => {
                debug_assert!(self.current_position.tunnels.contains(&label));

                let next_position = self.valves.get(&label).ok_or(Day16Error::ValveNotFound)?;
                self.current_position = next_position;
                Ok(())
            }
            Action::NoOp => Ok(()),
        }
    }
}

fn all_shortest_paths(
    valves: &HashMap<String, Valve>,
) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut distances: HashMap<(&String, &String), u64> = HashMap::new();
    let mut next: HashMap<(&String, &String), &String> = HashMap::new();

    for (from, valve) in valves {
        distances.insert((from, from), 0);
        next.insert((from, from), from);

        for to in &valve.tunnels {
            distances.insert((from, to), 1);
            next.insert((from, to), to);
        }
    }

    fn get_distance_through_k(
        distances: &HashMap<(&String, &String), u64>,
        k: &String,
        i: &String,
        j: &String,
    ) -> Option<u64> {
        let i_k = distances.get(&(i, k))?;
        let k_j = distances.get(&(k, j))?;

        Some(i_k + k_j)
    }

    for k in valves.keys() {
        for i in valves.keys() {
            for j in valves.keys() {
                let distance_through_k = get_distance_through_k(&distances, k, i, j);

                if let Some(i_k_j) = distance_through_k {
                    let found_better_path = match distances.get(&(i, j)) {
                        Some(&i_j) => i_j > i_k_j,
                        None => true,
                    };

                    if found_better_path {
                        distances.insert((i, j), i_k_j);
                        if let Some(n) = next.get(&(i, k)) {
                            next.insert((i, j), n);
                        }
                    }
                }
            }
        }
    }

    fn get_shortest_path<'a>(
        next: &HashMap<(&String, &String), &'a String>,
        mut from: &'a String,
        to: &'a String,
    ) -> Option<Vec<String>> {
        let mut path = vec![];

        while from != to {
            from = next.get(&(from, to))?;
            path.push(from.clone());
        }

        Some(path)
    }

    let mut result = HashMap::new();

    for from in valves.keys() {
        let mut result_from = HashMap::new();

        for to in valves.keys() {
            if let Some(path) = get_shortest_path(&next, from, to) {
                result_from.insert(to.clone(), path);
            }
        }

        result.insert(from.clone(), result_from);
    }

    result
}

fn generate_strategies(
    time_limit: usize,
    valves: &HashMap<String, Valve>,
    valves_to_open: HashSet<&Valve>,
    current_position: &String,
) -> Result<Vec<Vec<Action>>, Day16Error> {
    let shortest_paths = all_shortest_paths(valves);
    let mut strategies = vec![];

    fn aux(
        time_limit: usize,
        valves: &HashMap<String, Valve>,
        shortest_paths: &HashMap<String, HashMap<String, Vec<String>>>,
        current_position: &String,
        valves_to_open: HashSet<&Valve>,
        strategy: Vec<Action>,
        strategies: &mut Vec<Vec<Action>>,
    ) -> Result<(), Day16Error> {
        let mut stuck = true;

        if let Some(shortest_paths_from_current) = shortest_paths.get(current_position) {
            for next_valve in &valves_to_open {
                // for (next_position, next_valve) in valves {
                //     if !opened_valves.contains(next_position) && next_valve.flow_rate > 0 {
                let next_position = &next_valve.label;
                if let Some(path) = shortest_paths_from_current.get(next_position) {
                    if strategy.len() + path.len() + 1 <= time_limit {
                        let mut valves_to_open = valves_to_open.clone();
                        valves_to_open.remove(next_valve);

                        let mut strategy = strategy.clone();
                        strategy.extend(path.iter().map(|step| Action::MoveTo(step.clone())));
                        strategy.push(Action::OpenValve);

                        aux(
                            time_limit,
                            valves,
                            shortest_paths,
                            next_position,
                            valves_to_open,
                            strategy,
                            strategies,
                        )?;

                        stuck = false;
                    }
                }
            }
        }

        if stuck {
            let mut strategy = strategy;
            strategy.resize(time_limit, Action::NoOp);

            strategies.push(strategy);
        }

        Ok(())
    }

    aux(
        time_limit,
        valves,
        &shortest_paths,
        current_position,
        valves_to_open,
        vec![],
        &mut strategies,
    )?;

    Ok(strategies)
}

fn play_strategy(
    valves: &HashMap<String, Valve>,
    starting_position: &String,
    strategy: Vec<Action>,
) -> Result<u64, Day16Error> {
    let mut state = VolcanoState::new(valves, starting_position)?;

    for action in strategy {
        state.tick(action)?;
    }

    Ok(state.pressure_released)
}

fn part1(input: &Vec<String>) -> Result<u64, Day16Error> {
    let valves = parse_valves(input)?;
    let starting_position = String::from("AA");

    let valves_to_open = valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .collect();
    let strategies = generate_strategies(30, &valves, valves_to_open, &starting_position)?;

    let scores = strategies
        .into_iter()
        .map(|strategy| play_strategy(&valves, &starting_position, strategy))
        .collect::<Result<Vec<_>, _>>()?;

    scores.into_iter().max().ok_or(Day16Error::EmptyInput)
}

#[derive(Debug, Clone)]
struct VolcanoState2<'a> {
    valves: &'a HashMap<String, Valve>,
    my_position: &'a Valve,
    elephant_position: &'a Valve,
    open_valves: HashSet<&'a Valve>,
    pressure_released: u64,
}

impl<'a> VolcanoState2<'a> {
    fn new(
        valves: &'a HashMap<String, Valve>,
        starting_position: &String,
    ) -> Result<Self, Day16Error> {
        let current_position = valves
            .get(starting_position)
            .ok_or(Day16Error::EmptyInput)?;

        Ok(Self {
            valves,
            my_position: current_position,
            elephant_position: current_position,
            open_valves: HashSet::new(),
            pressure_released: 0,
        })
    }

    fn tick(&mut self, my_action: &Action, elephant_action: &Action) -> Result<(), Day16Error> {
        for v in &self.open_valves {
            self.pressure_released += v.flow_rate;
        }

        match my_action {
            Action::OpenValve => {
                self.open_valves.insert(self.my_position);
            }
            Action::MoveTo(label) => {
                debug_assert!(self.my_position.tunnels.contains(label));

                let next_position = self.valves.get(label).ok_or(Day16Error::ValveNotFound)?;
                self.my_position = next_position;
            }
            Action::NoOp => {}
        }

        match elephant_action {
            Action::OpenValve => {
                self.open_valves.insert(self.elephant_position);
            }
            Action::MoveTo(label) => {
                debug_assert!(self.elephant_position.tunnels.contains(label));

                let next_position = self.valves.get(label).ok_or(Day16Error::ValveNotFound)?;
                self.elephant_position = next_position;
            }
            Action::NoOp => {}
        }

        Ok(())
    }
}

fn play_strategy_2(
    valves: &HashMap<String, Valve>,
    starting_position: &String,
    my_actions: &Vec<Action>,
    elephant_actions: &Vec<Action>,
) -> Result<u64, Day16Error> {
    let mut state = VolcanoState2::new(valves, starting_position)?;

    for (my_action, elephant_action) in my_actions.iter().zip(elephant_actions) {
        state.tick(my_action, elephant_action)?;
    }

    Ok(state.pressure_released)
}

// fn partition_valves_to_open<'a>(valves_to_open: &HashSet<&'a Valve>) -> Vec<HashSet<&'a Valve>> {
//     (0..2usize.pow(valves_to_open.len() as u32))
//         .map(|i| {
//             valves_to_open
//                 .iter()
//                 .enumerate()
//                 .filter(|&(t, _)| (i >> t) % 2 == 1)
//                 .map(|(_, &valve)| valve)
//                 .collect()
//         })
//         .collect()
// }

fn part2(input: &Vec<String>) -> Result<u64, Day16Error> {
    let valves = parse_valves(input)?;
    let starting_position = String::from("AA");

    let mut max_score: Option<u64> = None;

    let valves_to_open: HashSet<_> = valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .collect();

    // TODO very slow (at least 30 minutes)
    let strategies = generate_strategies(26, &valves, valves_to_open, &starting_position)?;

    let total = strategies.len() * strategies.len();
    let mut last_percent = 0;
    let mut progress: usize = 0;

    for my_actions in &strategies {
        for elephant_actions in &strategies {
            let score = play_strategy_2(&valves, &starting_position, my_actions, elephant_actions)?;

            max_score = Some(max_score.map_or(score, |other| other.max(score)));

            progress += 1;
            let percent = (progress * 100) / total;
            if percent > last_percent {
                println!("{}% done", percent);
                last_percent = percent;
            }
        }
    }

    // Thought this would be better but it's actually a lot worse (couple hours)

    // let partition = partition_valves_to_open(&valves_to_open);

    // for my_valves_to_open in partition {
    //     if my_valves_to_open.len() == 0 || my_valves_to_open.len() * 2 > valves_to_open.len() {
    //         continue;
    //     }

    //     let elephant_valves_to_open = &valves_to_open - &my_valves_to_open;

    //     let my_strategies =
    //         generate_strategies(26, &valves, my_valves_to_open, &starting_position)?;
    //     let elephant_strategies =
    //         generate_strategies(26, &valves, elephant_valves_to_open, &starting_position)?;

    //     println!("{} {}", my_strategies.len(), elephant_strategies.len());

    //     for my_actions in &my_strategies {
    //         for elephant_actions in &elephant_strategies {
    //             let score =
    //                 play_strategy_2(&valves, &starting_position, my_actions, elephant_actions)?;

    //             max_score = Some(max_score.map_or(score, |other| other.max(score)));
    //         }
    //     }
    // }

    max_score.ok_or(Day16Error::EmptyInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_parse_valve_single_tunnel() {
        let valve: Valve = "Valve HH has flow rate=22; tunnel leads to valve GG"
            .parse()
            .unwrap();

        let expected = Valve {
            label: String::from("HH"),
            flow_rate: 22,
            tunnels: vec![String::from("GG")],
        };

        assert_eq!(valve, expected);
    }

    #[test]
    fn test_parse_valve_multiple_tunnels() {
        let valve: Valve = "Valve BB has flow rate=13; tunnels lead to valves CC, AA"
            .parse()
            .unwrap();

        let expected = Valve {
            label: String::from("BB"),
            flow_rate: 13,
            tunnels: vec![String::from("CC"), String::from("AA")],
        };

        assert_eq!(valve, expected);
    }

    #[test]
    fn test_shortest_paths() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();
        let valves = parse_valves(&input).unwrap();

        let shortest_paths = all_shortest_paths(&valves);

        assert_eq!(
            *shortest_paths
                .get(&String::from("AA"))
                .unwrap()
                .get(&String::from("HH"))
                .unwrap(),
            vec![
                String::from("DD"),
                String::from("EE"),
                String::from("FF"),
                String::from("GG"),
                String::from("HH")
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part1(&input).unwrap(), 1651);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part2(&input).unwrap(), 1707);
    }
}
