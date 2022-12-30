use std::io;

use aoc::read_lines;

#[derive(Debug)]
enum Day0Error {
    IoError(io::Error),
}

impl From<io::Error> for Day0Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day0.txt";

fn main() -> Result<(), Day0Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

fn part1(input: &Vec<String>) -> Result<usize, Day0Error> {
    todo!()
}

fn part2(input: &Vec<String>) -> Result<usize, Day0Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
ABCD
";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part1(&input).unwrap(), todo!());
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part2(&input).unwrap(), todo!());
    }
}
