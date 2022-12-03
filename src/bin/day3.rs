use std::{collections::HashSet, io};

use aoc::read_lines;

#[derive(Debug)]
enum Day3Error {
    IoError(io::Error),
    OddNumberOfItems,
    InvalidItem(u8),
    NoOverlappingItems,
    ManyOverlappingItems,
    EmptyGroup,
}

impl From<io::Error> for Day3Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day3.txt";

fn main() -> Result<(), Day3Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

type RucksackContents<'a> = (&'a [u8], &'a [u8]);

fn parse_rucksack_contents(line: &String) -> Result<RucksackContents, Day3Error> {
    let chars = line.as_bytes();
    let n = chars.len();

    if n % 2 != 0 {
        Err(Day3Error::OddNumberOfItems)
    } else {
        Ok(chars.split_at(n / 2))
    }
}

fn find_overlapping_item((first, second): RucksackContents) -> Result<u8, Day3Error> {
    let first: HashSet<_> = first.into_iter().collect();
    let second: HashSet<_> = second.into_iter().collect();

    let overlap: Vec<_> = first.intersection(&second).collect();
    match overlap.as_slice() {
        [] => Err(Day3Error::NoOverlappingItems),
        [&&item] => Ok(item),
        _ => Err(Day3Error::ManyOverlappingItems),
    }
}

fn get_priority(item: u8) -> Result<u8, Day3Error> {
    match item {
        b'a'..=b'z' => Ok(item + 1 - b'a'),
        b'A'..=b'Z' => Ok(item + 27 - b'A'),
        _ => Err(Day3Error::InvalidItem(item)),
    }
}

fn part1(input: &Vec<String>) -> Result<u32, Day3Error> {
    let priorities = input
        .iter()
        .map(parse_rucksack_contents)
        .map(|r| r.and_then(find_overlapping_item))
        .map(|r| r.and_then(get_priority))
        .collect::<Result<Vec<_>, Day3Error>>()?;

    let total_priority: u32 = priorities.into_iter().map(|p| p as u32).sum();

    Ok(total_priority)
}

fn to_byte_set(line: &String) -> HashSet<&u8> {
    line.as_bytes().iter().collect()
}

fn find_overlapping_item_for_group(group: &[HashSet<&u8>]) -> Result<u8, Day3Error> {
    let mut sets = group.iter();
    let mut overlap = sets.next().ok_or(Day3Error::EmptyGroup)?.clone();
    for set in sets {
        overlap.retain(|item| set.contains(item));
    }

    match overlap.iter().collect::<Vec<_>>().as_slice() {
        [] => Err(Day3Error::NoOverlappingItems),
        [&&item] => Ok(item),
        _ => Err(Day3Error::ManyOverlappingItems),
    }
}

fn part2(input: &Vec<String>) -> Result<u32, Day3Error> {
    let contents: Vec<_> = input.iter().map(to_byte_set).collect();
    let priorities = contents
        .chunks_exact(3)
        .map(find_overlapping_item_for_group)
        .map(|r| r.and_then(get_priority))
        .collect::<Result<Vec<_>, Day3Error>>()?;

    let total_priority: u32 = priorities.into_iter().map(|p| p as u32).sum();

    Ok(total_priority)
}
