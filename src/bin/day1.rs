use std::num::ParseIntError;

use aoc::{read_lines, AocError};

fn main() -> Result<(), AocError> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {:?}", part2()?);

    Ok(())
}

const INPUT_PATH: &str = "inputs/day1.txt";

fn part1() -> Result<u32, AocError> {
    let lines = read_lines(INPUT_PATH)?;
    let elf_calories = parse_elf_calories(lines)?;

    let elf_total_calories: Vec<u32> = elf_calories.into_iter().map(|v| v.iter().sum()).collect();
    let max_calories = elf_total_calories.into_iter().max();

    Ok(max_calories.unwrap())
}

fn part2() -> Result<u32, AocError> {
    let lines = read_lines(INPUT_PATH)?;
    let elf_calories = parse_elf_calories(lines)?;

    let mut elf_total_calories: Vec<u32> =
        elf_calories.into_iter().map(|v| v.iter().sum()).collect();

    elf_total_calories.sort_by(|a, b| b.cmp(a));

    Ok(elf_total_calories[0..3].iter().sum())
}

fn parse_elf_calories(lines: Vec<String>) -> Result<Vec<Vec<u32>>, ParseIntError> {
    lines
        .split(|line| line.is_empty())
        .map(|v| v.iter().map(|s| s.parse()).collect())
        .collect()
}
