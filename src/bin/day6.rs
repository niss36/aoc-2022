use std::{
    collections::{HashSet, VecDeque},
    io,
};

use aoc::read_lines;

#[derive(Debug)]
enum Day6Error {
    IoError(io::Error),
    EmptyInput,
    NoMarker,
}

impl From<io::Error> for Day6Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day6.txt";

fn main() -> Result<(), Day6Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

fn find_marker(input: &str, window_size: usize) -> Result<usize, Day6Error> {
    let mut chars = input.chars().enumerate();
    let mut window: VecDeque<char> = VecDeque::new();
    for _ in 0..(window_size - 1) {
        if let Some((_, c)) = chars.next() {
            window.push_back(c);
        }
    }

    for (i, c) in chars {
        debug_assert_eq!(window.len(), window_size - 1);

        window.push_back(c);

        let window_set: HashSet<_> = window.iter().collect();
        if window_set.len() == window.len() {
            // All items are different
            return Ok(i + 1);
        }

        window.pop_front();
    }

    Err(Day6Error::NoMarker)
}

fn part1(input: &Vec<String>) -> Result<usize, Day6Error> {
    let input = input.first().ok_or(Day6Error::EmptyInput)?;

    find_marker(input, 4)
}

fn part2(input: &Vec<String>) -> Result<usize, Day6Error> {
    let input = input.first().ok_or(Day6Error::EmptyInput)?;

    find_marker(input, 14)
}
