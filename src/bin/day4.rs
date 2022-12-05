use std::{collections::HashSet, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day4Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidLine(String),
    InvalidRange(String),
}

impl From<io::Error> for Day4Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day4Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day4.txt";

fn main() -> Result<(), Day4Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

struct ElfAssignmentPair(HashSet<u32>, HashSet<u32>);

impl FromStr for ElfAssignmentPair {
    type Err = Day4Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn set_from_str(s: &str) -> Result<HashSet<u32>, Day4Error> {
            let v: Vec<_> = s.split("-").collect();
            match v.as_slice() {
                [start, end] => {
                    let start: u32 = start.parse()?;
                    let end: u32 = end.parse()?;

                    Ok((start..=end).collect())
                }
                _ => Err(Day4Error::InvalidRange(s.to_owned())),
            }
        }

        let v: Vec<_> = s.split(",").collect();
        match v.as_slice() {
            [a, b] => Ok(ElfAssignmentPair(set_from_str(a)?, set_from_str(b)?)),
            _ => Err(Self::Err::InvalidLine(s.to_owned())),
        }
    }
}

fn parse_assignment_pairs(input: &Vec<String>) -> Result<Vec<ElfAssignmentPair>, Day4Error> {
    input.iter().map(|line| line.parse()).collect()
}

fn is_fully_contained(ElfAssignmentPair(assignment1, assignment2): &ElfAssignmentPair) -> bool {
    assignment1.is_superset(assignment2) || assignment2.is_superset(assignment1)
}

fn part1(input: &Vec<String>) -> Result<usize, Day4Error> {
    Ok(parse_assignment_pairs(input)?
        .into_iter()
        .filter(is_fully_contained)
        .count())
}

fn is_overlapping(ElfAssignmentPair(assignment1, assignment2): &ElfAssignmentPair) -> bool {
    !assignment1.is_disjoint(assignment2)
}

fn part2(input: &Vec<String>) -> Result<usize, Day4Error> {
    Ok(parse_assignment_pairs(input)?
        .into_iter()
        .filter(is_overlapping)
        .count())
}
