use std::{collections::VecDeque, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day10Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidInstruction,
}

impl From<io::Error> for Day10Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day10Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day10.txt";

fn main() -> Result<(), Day10Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: \n{}", part2(&input)?);

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(isize),
}

impl FromStr for Instruction {
    type Err = Day10Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(" ").collect();
        match v.as_slice() {
            ["noop"] => Ok(Self::Noop),
            ["addx", value] => Ok(Self::AddX(value.parse()?)),
            _ => Err(Self::Err::InvalidInstruction),
        }
    }
}

fn parse_instructions(input: &Vec<String>) -> Result<VecDeque<Instruction>, Day10Error> {
    input.iter().map(|line| line.parse()).collect()
}

impl Instruction {
    fn cycles_to_complete(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn with_cycles_to_complete(self) -> (Self, usize) {
        let cycles_to_complete = self.cycles_to_complete();
        (self, cycles_to_complete)
    }
}

#[derive(Debug)]
struct State {
    cycle_number: usize,
    x_register_value: isize,
    in_progress: Option<(Instruction, usize)>,
}

impl State {
    fn new() -> Self {
        Self {
            cycle_number: 0,
            x_register_value: 1,
            in_progress: None,
        }
    }

    fn begin_tick(mut self, instructions: &mut VecDeque<Instruction>) -> Self {
        self.cycle_number += 1;

        match self.in_progress {
            Some(_) => {}
            None => {
                self.in_progress = instructions
                    .pop_front()
                    .map(Instruction::with_cycles_to_complete)
            }
        }

        self
    }

    fn end_tick(mut self) -> Self {
        match self.in_progress {
            Some((instruction, cycles_left)) => {
                if cycles_left <= 1 {
                    match instruction {
                        Instruction::Noop => {}
                        Instruction::AddX(value) => self.x_register_value += value,
                    }

                    self.in_progress = None;
                } else {
                    self.in_progress = Some((instruction, cycles_left - 1));
                }
            }
            None => {}
        }

        self
    }

    fn signal_strength(&self) -> isize {
        (self.cycle_number as isize) * self.x_register_value
    }
}

fn part1(input: &Vec<String>) -> Result<isize, Day10Error> {
    let mut instructions = parse_instructions(input)?;

    let mut state = State::new();
    let mut total_signal_strength = 0;

    while !instructions.is_empty() {
        state = state.begin_tick(&mut instructions);

        if state.cycle_number % 40 == 20 && state.cycle_number <= 220 {
            total_signal_strength += state.signal_strength();
        }

        state = state.end_tick();
    }

    Ok(total_signal_strength)
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const CRT_AREA: usize = CRT_WIDTH * CRT_HEIGHT;

struct Crt {
    display: [bool; CRT_AREA],
}

impl Crt {
    fn new() -> Self {
        Self {
            display: [false; CRT_AREA],
        }
    }

    fn update(mut self, state: &State) -> Self {
        let index = state.cycle_number - 1;
        let is_lit = state
            .x_register_value
            .abs_diff((index % CRT_WIDTH) as isize)
            <= 1;

        self.display[index % CRT_AREA] = is_lit;

        self
    }

    fn to_str(&self) -> String {
        let mut s = String::with_capacity(CRT_AREA + CRT_HEIGHT);
        for i in 0..CRT_HEIGHT {
            let row = &self.display[(i * CRT_WIDTH)..((i + 1) * CRT_WIDTH)];

            s.extend(row.iter().map(|b| if *b { '#' } else { '.' }));
            s.push('\n');
        }

        s
    }
}

fn part2(input: &Vec<String>) -> Result<String, Day10Error> {
    let mut instructions = parse_instructions(input)?;

    let mut state = State::new();
    let mut crt = Crt::new();

    while !instructions.is_empty() {
        state = state.begin_tick(&mut instructions);

        crt = crt.update(&state);

        state = state.end_tick();
    }

    Ok(crt.to_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(part1(&input).unwrap(), 13140);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(
            part2(&input).unwrap(),
            String::from(
                "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            )
        );
    }
}
