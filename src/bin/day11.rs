use std::{collections::VecDeque, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day11Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidMonkeyOperation,
    InvalidMonkeyFormat,
}

impl From<io::Error> for Day11Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day11Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day11.txt";

fn main() -> Result<(), Day11Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
enum MonkeyOperation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl FromStr for MonkeyOperation {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(" ").collect();
        match v.as_slice() {
            ["old", "*", "old"] => Ok(Self::Square),
            ["old", "*", n] => Ok(Self::Multiply(n.parse()?)),
            ["old", "+", n] => Ok(Self::Add(n.parse()?)),
            _ => Err(Self::Err::InvalidMonkeyOperation),
        }
    }
}

impl MonkeyOperation {
    fn calculate(&self, old: usize) -> usize {
        use MonkeyOperation::*;

        match self {
            Add(n) => old + n,
            Multiply(n) => old * n,
            Square => old * old,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct MonkeyTest {
    if_divisible_by: usize,
    then_throw_to: usize,
    else_throw_to: usize,
}

impl TryFrom<&[String]> for MonkeyTest {
    type Error = Day11Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value {
            [if_divisible_by, then_throw_to, else_throw_to] => {
                let if_divisible_by = if_divisible_by
                    .strip_prefix("  Test: divisible by ")
                    .ok_or(Self::Error::InvalidMonkeyFormat)?
                    .parse()?;

                let then_throw_to = then_throw_to
                    .strip_prefix("    If true: throw to monkey ")
                    .ok_or(Self::Error::InvalidMonkeyFormat)?
                    .parse()?;

                let else_throw_to = else_throw_to
                    .strip_prefix("    If false: throw to monkey ")
                    .ok_or(Self::Error::InvalidMonkeyFormat)?
                    .parse()?;

                Ok(Self {
                    if_divisible_by,
                    then_throw_to,
                    else_throw_to,
                })
            }
            _ => Err(Self::Error::InvalidMonkeyFormat),
        }
    }
}

impl MonkeyTest {
    fn apply(&self, item: usize) -> usize {
        if item % self.if_divisible_by == 0 {
            self.then_throw_to
        } else {
            self.else_throw_to
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: MonkeyOperation,
    test: MonkeyTest,
}

impl TryFrom<&[String]> for Monkey {
    type Error = Day11Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value {
            [_, items, operation, test @ ..] => {
                let items = items
                    .strip_prefix("  Starting items: ")
                    .ok_or(Self::Error::InvalidMonkeyFormat)?;

                let items = items
                    .split(", ")
                    .map(|item| item.parse())
                    .collect::<Result<_, _>>()?;

                let operation = operation
                    .strip_prefix("  Operation: new = ")
                    .ok_or(Self::Error::InvalidMonkeyFormat)?;

                Ok(Self {
                    items,
                    operation: operation.parse()?,
                    test: test.try_into()?,
                })
            }
            _ => Err(Self::Error::InvalidMonkeyFormat),
        }
    }
}

struct ThrownItem {
    item: usize,
    thrown_to: usize,
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<ThrownItem> {
        self.items
            .drain(..)
            .map(|item| {
                let item = self.operation.calculate(item) / 3;

                ThrownItem {
                    item,
                    thrown_to: self.test.apply(item),
                }
            })
            .collect()
    }

    fn take_turn_2(&mut self, modulo: usize) -> Vec<ThrownItem> {
        self.items
            .drain(..)
            .map(|item| {
                let item = self.operation.calculate(item) % modulo;

                ThrownItem {
                    item,
                    thrown_to: self.test.apply(item),
                }
            })
            .collect()
    }
}

fn parse_monkeys(input: &Vec<String>) -> Result<Vec<Monkey>, Day11Error> {
    input
        .split(|line| line.is_empty())
        .map(|lines| lines.try_into())
        .collect()
}

fn monkey_business(mut activity: Vec<usize>) -> usize {
    activity.sort_by(|a, b| b.cmp(a));

    activity[0] * activity[1]
}

fn part1(input: &Vec<String>) -> Result<usize, Day11Error> {
    let mut monkeys = parse_monkeys(input)?;
    let mut activity: Vec<usize> = monkeys.iter().map(|_| 0).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].take_turn();
            activity[i] += thrown_items.len();

            for ThrownItem { item, thrown_to } in thrown_items {
                monkeys[thrown_to].items.push_back(item);
            }
        }
    }

    Ok(monkey_business(activity))
}

fn part2(input: &Vec<String>) -> Result<usize, Day11Error> {
    let mut monkeys = parse_monkeys(input)?;
    let mut activity: Vec<usize> = monkeys.iter().map(|_| 0).collect();

    let modulo: usize = monkeys
        .iter()
        .map(|monkey| monkey.test.if_divisible_by)
        .product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].take_turn_2(modulo);
            activity[i] += thrown_items.len();

            for ThrownItem { item, thrown_to } in thrown_items {
                monkeys[thrown_to].items.push_back(item);
            }
        }
    }

    Ok(monkey_business(activity))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let input: Vec<String> = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(
            Monkey {
                items: vec![79, 98].into(),
                operation: MonkeyOperation::Multiply(19),
                test: MonkeyTest {
                    if_divisible_by: 23,
                    then_throw_to: 2,
                    else_throw_to: 3
                }
            },
            input.as_slice().try_into().unwrap()
        );
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(part1(&input).unwrap(), 10605);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(part2(&input).unwrap(), 2713310158);
    }
}
