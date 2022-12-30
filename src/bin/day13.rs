use std::{cmp::Ordering, collections::VecDeque, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day13Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidPackageValue,
    InvalidNumberOfPackets,
    DividerNotFound,
}

impl From<io::Error> for Day13Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day13Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day13.txt";

fn main() -> Result<(), Day13Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketValue {
    Integer(u32),
    List(Vec<PacketValue>),
}

fn parse_packet_value(buffer: &mut VecDeque<char>) -> Result<PacketValue, Day13Error> {
    match buffer.front() {
        Some('[') => {
            buffer.pop_front();

            let mut v = vec![];

            loop {
                if let Some(']') = buffer.front() {
                    buffer.pop_front();
                    break;
                }

                v.push(parse_packet_value(buffer)?);

                match buffer.pop_front() {
                    Some(',') => {}
                    Some(']') => break,
                    _ => return Err(Day13Error::InvalidPackageValue),
                }
            }

            Ok(PacketValue::List(v))
        }
        Some(_) => {
            let mut s = String::new();
            while let Some(&c) = buffer.front() {
                if c != ']' && c != ',' {
                    buffer.pop_front();
                    s.push(c);
                } else {
                    break;
                }
            }

            Ok(PacketValue::Integer(s.parse()?))
        }
        None => Err(Day13Error::InvalidPackageValue),
    }
}

impl FromStr for PacketValue {
    type Err = Day13Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buffer = s.chars().collect();
        parse_packet_value(&mut buffer)
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        use PacketValue::*;

        fn compare_lists(left: &Vec<PacketValue>, right: &Vec<PacketValue>) -> Ordering {
            for (i, left_value) in left.iter().enumerate() {
                if let Some(right_value) = right.get(i) {
                    match left_value.cmp(right_value) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                    }
                } else {
                    return Ordering::Greater;
                }
            }

            if left.len() == right.len() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        }

        match self {
            Integer(left) => match other {
                Integer(right) => left.cmp(right),
                List(right) => {
                    let left = vec![Integer(*left)];
                    compare_lists(&left, right)
                }
            },
            List(left) => match other {
                Integer(right) => {
                    let right = vec![Integer(*right)];
                    compare_lists(left, &right)
                }
                List(right) => compare_lists(left, right),
            },
        }
    }
}

fn parse_packet_pair(lines: &[String]) -> Result<(PacketValue, PacketValue), Day13Error> {
    match lines {
        [left, right] => Ok((left.parse()?, right.parse()?)),
        _ => Err(Day13Error::InvalidNumberOfPackets),
    }
}

fn parse_packet_pairs(input: &Vec<String>) -> Result<Vec<(PacketValue, PacketValue)>, Day13Error> {
    input
        .split(|line| line.is_empty())
        .map(|lines| parse_packet_pair(lines))
        .collect()
}

fn part1(input: &Vec<String>) -> Result<usize, Day13Error> {
    let packet_pairs = parse_packet_pairs(input)?;

    Ok(packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.cmp(right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum())
}

fn parse_packets(input: &Vec<String>) -> Result<Vec<PacketValue>, Day13Error> {
    input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect()
}

fn part2(input: &Vec<String>) -> Result<usize, Day13Error> {
    let mut packets = parse_packets(input)?;
    let divider_a: PacketValue = "[[2]]".parse()?;
    let divider_b: PacketValue = "[[6]]".parse()?;

    packets.push(divider_a.clone());
    packets.push(divider_b.clone());

    packets.sort();

    let mut divider_a_index = None;
    let mut divider_b_index = None;

    for (index, packet) in packets.iter().enumerate() {
        if *packet == divider_a {
            divider_a_index = Some(index + 1);
        }

        if *packet == divider_b {
            divider_b_index = Some(index + 1);
        }
    }

    let divider_a_index = divider_a_index.ok_or(Day13Error::DividerNotFound)?;
    let divider_b_index = divider_b_index.ok_or(Day13Error::DividerNotFound)?;

    Ok(divider_a_index * divider_b_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    const EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_parse_packet() {
        let value: PacketValue = "[[1],[2,3,4]]".parse().unwrap();
        let expected = PacketValue::List(vec![
            PacketValue::List(vec![PacketValue::Integer(1)]),
            PacketValue::List(vec![
                PacketValue::Integer(2),
                PacketValue::Integer(3),
                PacketValue::Integer(4),
            ]),
        ]);

        assert_eq!(value, expected)
    }

    #[test]
    fn test_packet_cmp() {
        let left: PacketValue = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
        let right: PacketValue = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 140);
    }
}
