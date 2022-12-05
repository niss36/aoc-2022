use std::{io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day5Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidFormat,
    EmptyInput,
    InvalidLine(String),
    EmptyStack,
}

impl From<io::Error> for Day5Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day5Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day5.txt";

fn main() -> Result<(), Day5Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct CrateArrangement(Vec<Vec<char>>);

impl TryFrom<&[String]> for CrateArrangement {
    type Error = Day5Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let (last, rest) = value.split_last().ok_or(Self::Error::EmptyInput)?;

        let number_stacks = last.chars().filter(|c| *c != ' ').count();
        let mut crate_arrangement: Vec<Vec<char>> = (0..number_stacks).map(|_| vec![]).collect();

        for line in rest.iter().rev() {
            for stack_index in 0..number_stacks {
                let line_index = stack_index * 4 + 1;
                match line.chars().nth(line_index) {
                    Some(' ') => continue,
                    Some(c) => crate_arrangement[stack_index].push(c),
                    None => return Err(Self::Error::InvalidLine(line.to_owned())),
                }
            }
        }

        Ok(CrateArrangement(crate_arrangement))
    }
}

#[derive(Debug)]
struct Step {
    number: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = Day5Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(" ").collect();
        match v.as_slice() {
            ["move", number, "from", from, "to", to] => Ok(Step {
                number: number.parse()?,
                from: from.parse()?,
                to: to.parse()?,
            }),
            _ => Err(Self::Err::InvalidLine(s.to_owned())),
        }
    }
}

fn parse_steps(lines: &[String]) -> Result<Vec<Step>, Day5Error> {
    lines.iter().map(|line| line.parse()).collect()
}

fn parse_crate_arrangement_and_steps(
    input: &Vec<String>,
) -> Result<(CrateArrangement, Vec<Step>), Day5Error> {
    let v: Vec<_> = input.split(|line| line.is_empty()).collect();

    match v.as_slice() {
        [crates, steps] => Ok(((*crates).try_into()?, parse_steps(steps)?)),
        _ => Err(Day5Error::InvalidFormat),
    }
}

fn apply_step(
    CrateArrangement(mut crate_arrangement): CrateArrangement,
    Step { number, from, to }: Step,
) -> Result<CrateArrangement, Day5Error> {
    for _ in 0..number {
        let c = crate_arrangement[from - 1]
            .pop()
            .ok_or(Day5Error::EmptyStack)?;

        crate_arrangement[to - 1].push(c);
    }

    Ok(CrateArrangement(crate_arrangement))
}

fn top_crates(CrateArrangement(crate_arrangement): CrateArrangement) -> Result<String, Day5Error> {
    crate_arrangement
        .iter()
        .map(|stack| stack.last().ok_or(Day5Error::EmptyStack))
        .collect()
}

fn part1(input: &Vec<String>) -> Result<String, Day5Error> {
    let (mut crate_arrangement, steps) = parse_crate_arrangement_and_steps(input)?;

    for step in steps {
        crate_arrangement = apply_step(crate_arrangement, step)?;
    }

    top_crates(crate_arrangement)
}

fn apply_step_2(
    CrateArrangement(mut crate_arrangement): CrateArrangement,
    Step { number, from, to }: Step,
) -> Result<CrateArrangement, Day5Error> {
    let from_stack = &mut crate_arrangement[from - 1];
    let mut crates = from_stack.split_off(from_stack.len() - number);

    let to_stack = &mut crate_arrangement[to - 1];
    to_stack.append(&mut crates);

    Ok(CrateArrangement(crate_arrangement))
}

fn part2(input: &Vec<String>) -> Result<String, Day5Error> {
    let (mut crate_arrangement, steps) = parse_crate_arrangement_and_steps(input)?;

    for step in steps {
        crate_arrangement = apply_step_2(crate_arrangement, step)?;
    }

    top_crates(crate_arrangement)
}
