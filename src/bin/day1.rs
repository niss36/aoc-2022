use std::{io, num::ParseIntError};

use aoc::read_lines;

#[derive(Debug)]
enum Day1Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    EmptyInput,
}

impl From<io::Error> for Day1Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day1Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day1.txt";

fn main() -> Result<(), Day1Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

fn part1(input: &Vec<String>) -> Result<u32, Day1Error> {
    let elf_calories = parse_elf_calories(input)?;

    let elf_total_calories: Vec<u32> = elf_calories.into_iter().map(|v| v.iter().sum()).collect();
    let max_calories = elf_total_calories.into_iter().max();

    max_calories.ok_or(Day1Error::EmptyInput)
}

fn part2(input: &Vec<String>) -> Result<u32, Day1Error> {
    let elf_calories = parse_elf_calories(input)?;

    let mut elf_total_calories: Vec<u32> =
        elf_calories.into_iter().map(|v| v.iter().sum()).collect();

    elf_total_calories.sort_by(|a, b| b.cmp(a));

    Ok(elf_total_calories[0..3].iter().sum())
}

fn parse_elf_calories(lines: &Vec<String>) -> Result<Vec<Vec<u32>>, ParseIntError> {
    lines
        .split(|line| line.is_empty())
        .map(|v| v.iter().map(|s| s.parse()).collect())
        .collect()
}
