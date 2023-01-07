use std::{collections::VecDeque, io, num::ParseIntError};

use aoc::read_lines;

#[derive(Debug)]
enum Day20Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    ZeroNotFound,
}

impl From<io::Error> for Day20Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day20Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day20.txt";

fn main() -> Result<(), Day20Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input, 811589153, 10)?);

    Ok(())
}

fn parse_encrypted_file(input: &Vec<String>) -> Result<Vec<i64>, ParseIntError> {
    input.iter().map(|line| line.parse()).collect()
}

fn move_around(i: &mut usize, index: usize, new_index: usize, n: usize) {
    if *i == index {
        *i = new_index;
    } else {
        if index < new_index {
            if index < *i && *i <= new_index {
                *i = (*i + n - 1).rem_euclid(n);
            }
        } else {
            if new_index <= *i && *i < index {
                *i = (*i + 1).rem_euclid(n);
            }
        }
    }
}

fn resulting_indices_to_file(
    encrypted_file: &Vec<i64>,
    resulting_indices: &Vec<usize>,
) -> Vec<i64> {
    let mut result = encrypted_file.clone();
    for (index, new_index) in resulting_indices.iter().enumerate() {
        result[*new_index] = encrypted_file[index];
    }

    result
}

fn mix(encrypted_file: Vec<i64>, decryption_key: i64, mixing_rounds: usize) -> Vec<i64> {
    let encrypted_file: Vec<_> = encrypted_file
        .into_iter()
        .map(|value| value * decryption_key)
        .collect();
    let n = encrypted_file.len();

    let mut next_indices_and_original_indices_and_values: VecDeque<_> =
        std::iter::repeat(encrypted_file.iter().enumerate().enumerate())
            .take(mixing_rounds)
            .flatten()
            .collect();

    // map current index to new index
    let mut resulting_indices: Vec<_> = (0..n).collect();

    while let Some((index, (original_index, value))) =
        next_indices_and_original_indices_and_values.pop_front()
    {
        let new_index = ((index as i64) + value).rem_euclid(n as i64 - 1) as usize;

        for (i, _) in next_indices_and_original_indices_and_values.iter_mut() {
            move_around(i, index, new_index, n);
        }

        assert_eq!(resulting_indices[original_index], index);

        for i in resulting_indices.iter_mut() {
            move_around(i, index, new_index, n);
        }
    }

    resulting_indices_to_file(&encrypted_file, &resulting_indices)
}

fn grove_coordinates(mixed_encrypted_file: Vec<i64>) -> Result<i64, Day20Error> {
    let zero_index = mixed_encrypted_file
        .iter()
        .position(|x| *x == 0)
        .ok_or(Day20Error::ZeroNotFound)?;

    Ok(
        mixed_encrypted_file[(zero_index + 1000) % mixed_encrypted_file.len()]
            + mixed_encrypted_file[(zero_index + 2000) % mixed_encrypted_file.len()]
            + mixed_encrypted_file[(zero_index + 3000) % mixed_encrypted_file.len()],
    )
}

fn part1(input: &Vec<String>) -> Result<i64, Day20Error> {
    let encrypted_file = parse_encrypted_file(input)?;
    let mixed_encrypted_file = mix(encrypted_file, 1, 1);

    grove_coordinates(mixed_encrypted_file)
}

fn part2(
    input: &Vec<String>,
    decryption_key: i64,
    mixing_rounds: usize,
) -> Result<i64, Day20Error> {
    let encrypted_file = parse_encrypted_file(input)?;
    let mixed_encrypted_file = mix(encrypted_file, decryption_key, mixing_rounds);

    grove_coordinates(mixed_encrypted_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4
";

    #[test]
    fn test_mix_1_1() {
        let encrypted_file = vec![1, 2, -3, 3, -2, 0, 4];
        let result = mix(encrypted_file, 1, 1);

        let expected_result = vec![-2, 1, 2, -3, 4, 0, 3];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mix_811589153_10() {
        let encrypted_file = vec![1, 2, -3, 3, -2, 0, 4];
        let result = mix(encrypted_file, 811589153, 10);

        let expected_result = vec![
            0,
            -2434767459,
            1623178306,
            3246356612,
            -1623178306,
            2434767459,
            811589153,
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_grove_coordinates() {
        let mixed_encrypted_file = vec![1, 2, -3, 4, 0, 3, -2];
        let result = grove_coordinates(mixed_encrypted_file).unwrap();

        let expected_result = 3;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input, 811589153, 10).unwrap(), 1623178306);
    }
}
