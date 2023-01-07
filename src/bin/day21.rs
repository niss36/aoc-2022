use std::{
    collections::{HashMap, HashSet},
    io,
    num::ParseIntError,
    str::FromStr,
};

use aoc::read_lines;

#[derive(Debug)]
enum Day21Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidOperation,
    InvalidMonkeyJob,
    InvalidMonkeyLine,
    MonkeyNotFound,
    UnexpectedRootJob,
    MoreThanOneHuman,
    SolveEquationError,
}

impl From<io::Error> for Day21Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day21Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day21.txt";

fn main() -> Result<(), Day21Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl FromStr for Operation {
    type Err = Day21Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),
            _ => Err(Self::Err::InvalidOperation),
        }
    }
}

impl Operation {
    fn compute(&self, left: &i64, right: &i64) -> i64 {
        use Operation::*;

        match self {
            Addition => left + right,
            Subtraction => left - right,
            Multiplication => left * right,
            Division => left / right,
        }
    }

    fn solve_left(self, left: i64, target: i64) -> i64 {
        // left (self) x == target
        // <=> x == self.solve_left(left, target)
        use Operation::*;

        match self {
            Addition => target - left,       // l + x == t ==> x = t - l
            Subtraction => left - target,    // l - x == t ==> x = l - t
            Multiplication => target / left, // l * x == t ==> x = t / l
            Division => left / target,       // l / x == t ==> x = l / t
        }
    }

    fn solve_right(self, right: i64, target: i64) -> i64 {
        // x (self) right == target
        // <=> x == self.solve_right(right, target)
        use Operation::*;

        match self {
            Addition => target - right,       // x + r = t ==> x = t - r
            Subtraction => target + right,    // x - r = t ==> x = t + r
            Multiplication => target / right, // x * r = t ==> x = t / r
            Division => target * right,       // x / r = t ==> x = t * r
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MonkeyJob {
    YellNumber(i64),
    YellOperation(Operation, String, String),
}

impl MonkeyJob {
    fn try_compute(&self, yelled_numbers: &HashMap<String, i64>) -> Option<i64> {
        use MonkeyJob::*;

        match self {
            YellNumber(number) => Some(*number),
            YellOperation(op, left, right) => {
                let left = yelled_numbers.get(left)?;
                let right = yelled_numbers.get(right)?;

                Some(op.compute(left, right))
            }
        }
    }
}

impl FromStr for MonkeyJob {
    type Err = Day21Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(' ').collect();
        match v.as_slice() {
            [number] => Ok(Self::YellNumber(number.parse()?)),
            [left, op, right] => Ok(Self::YellOperation(
                op.parse()?,
                left.to_string(),
                right.to_string(),
            )),
            _ => Err(Self::Err::InvalidMonkeyJob),
        }
    }
}

fn parse_monkeys(input: &Vec<String>) -> Result<HashMap<String, MonkeyJob>, Day21Error> {
    fn parse_line(line: &String) -> Result<(String, MonkeyJob), Day21Error> {
        let v: Vec<_> = line.split(": ").collect();
        match v.as_slice() {
            [name, job] => Ok((name.to_string(), job.parse()?)),
            _ => Err(Day21Error::InvalidMonkeyLine),
        }
    }

    input.iter().map(parse_line).collect()
}

fn compute_root_yelled_number(monkeys: HashMap<String, MonkeyJob>) -> Result<i64, Day21Error> {
    let mut yelled_numbers: HashMap<String, i64> = HashMap::new();

    let mut waiting_monkeys: HashSet<_> = monkeys.keys().collect();

    while !waiting_monkeys.is_empty() {
        let mut next_waiting_monkeys = waiting_monkeys.clone();

        for &monkey_name in &waiting_monkeys {
            let monkey_job = monkeys.get(monkey_name).ok_or(Day21Error::MonkeyNotFound)?;

            if let Some(yelled_number) = monkey_job.try_compute(&yelled_numbers) {
                next_waiting_monkeys.remove(monkey_name);
                yelled_numbers.insert(monkey_name.clone(), yelled_number);
            }
        }

        waiting_monkeys = next_waiting_monkeys;
    }

    yelled_numbers
        .get("root")
        .copied()
        .ok_or(Day21Error::MonkeyNotFound)
}

fn part1(input: &Vec<String>) -> Result<i64, Day21Error> {
    let monkeys = parse_monkeys(input)?;

    compute_root_yelled_number(monkeys)
}

#[derive(Debug)]
enum Expression {
    Human,
    Number(i64),
    Operation(Operation, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn reduce(self) -> Self {
        use Expression::*;

        match self {
            Operation(op, mut left, mut right) => {
                *left = (*left).reduce();
                *right = (*right).reduce();

                if let Number(left) = *left {
                    if let Number(right) = *right {
                        return Number(op.compute(&left, &right));
                    }
                }

                Operation(op, left, right)
            }
            other => other,
        }
    }
}

fn from_monkey_name(
    monkeys: &HashMap<String, MonkeyJob>,
    monkey_name: &String,
) -> Result<Expression, Day21Error> {
    use Expression::*;
    use MonkeyJob::*;

    if monkey_name == "humn" {
        Ok(Human)
    } else {
        match monkeys.get(monkey_name) {
            Some(YellNumber(number)) => Ok(Number(*number)),
            Some(YellOperation(op, left, right)) => Ok(Operation(
                *op,
                Box::new(from_monkey_name(monkeys, left)?),
                Box::new(from_monkey_name(monkeys, right)?),
            )),
            None => Err(Day21Error::MonkeyNotFound),
        }
    }
}

fn from_monkeys(
    monkeys: HashMap<String, MonkeyJob>,
) -> Result<(Expression, Expression), Day21Error> {
    let root = monkeys.get("root").ok_or(Day21Error::MonkeyNotFound)?;

    if let MonkeyJob::YellOperation(_, left, right) = root {
        Ok((
            from_monkey_name(&monkeys, left)?,
            from_monkey_name(&monkeys, right)?,
        ))
    } else {
        Err(Day21Error::UnexpectedRootJob)
    }
}

fn solve_equation((left, right): (Expression, Expression)) -> Result<i64, Day21Error> {
    fn solve_aux(expression: Expression, target: i64) -> Result<i64, Day21Error> {
        use Expression::*;

        match expression {
            Human => Ok(target),
            Number(_) => Err(Day21Error::SolveEquationError),
            Operation(op, left, right) => {
                if let Number(n) = *left {
                    solve_aux(*right, op.solve_left(n, target))
                } else if let Number(n) = *right {
                    solve_aux(*left, op.solve_right(n, target))
                } else {
                    Err(Day21Error::MoreThanOneHuman)
                }
            }
        }
    }

    let left = left.reduce();
    let right = right.reduce();

    if let Expression::Number(target) = left {
        solve_aux(right, target)
    } else if let Expression::Number(target) = right {
        solve_aux(left, target)
    } else {
        Err(Day21Error::MoreThanOneHuman)
    }
}

fn part2(input: &Vec<String>) -> Result<i64, Day21Error> {
    let monkeys = parse_monkeys(input)?;
    let equation = from_monkeys(monkeys)?;

    solve_equation(equation)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    #[test]
    fn test_parse_monkeys() {
        use MonkeyJob::*;
        use Operation::*;

        let input = to_lines(EXAMPLE);

        let result = parse_monkeys(&input).unwrap();
        let mut expected_result = HashMap::new();
        expected_result.insert(
            String::from("root"),
            YellOperation(Addition, String::from("pppw"), String::from("sjmn")),
        );
        expected_result.insert(String::from("dbpl"), YellNumber(5));
        expected_result.insert(
            String::from("cczh"),
            YellOperation(Addition, String::from("sllz"), String::from("lgvd")),
        );
        expected_result.insert(String::from("zczc"), YellNumber(2));
        expected_result.insert(
            String::from("ptdq"),
            YellOperation(Subtraction, String::from("humn"), String::from("dvpt")),
        );
        expected_result.insert(String::from("dvpt"), YellNumber(3));
        expected_result.insert(String::from("lfqf"), YellNumber(4));
        expected_result.insert(String::from("humn"), YellNumber(5));
        expected_result.insert(String::from("ljgn"), YellNumber(2));
        expected_result.insert(
            String::from("sjmn"),
            YellOperation(Multiplication, String::from("drzm"), String::from("dbpl")),
        );
        expected_result.insert(String::from("sllz"), YellNumber(4));
        expected_result.insert(
            String::from("pppw"),
            YellOperation(Division, String::from("cczh"), String::from("lfqf")),
        );
        expected_result.insert(
            String::from("lgvd"),
            YellOperation(Multiplication, String::from("ljgn"), String::from("ptdq")),
        );
        expected_result.insert(
            String::from("drzm"),
            YellOperation(Subtraction, String::from("hmdt"), String::from("zczc")),
        );
        expected_result.insert(String::from("hmdt"), YellNumber(32));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 152);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 301);
    }
}
