use std::str::FromStr;

use aoc::{read_lines, AocError, InputError};

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse_opponent_move(s: &str) -> Result<Move, InputError> {
    match s {
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        _ => Err(InputError::InvalidValue),
    }
}

fn parse_our_move(s: &str) -> Result<Move, InputError> {
    match s {
        "X" => Ok(Move::Rock),
        "Y" => Ok(Move::Paper),
        "Z" => Ok(Move::Scissors),
        _ => Err(InputError::InvalidValue),
    }
}

fn main() -> Result<(), AocError> {
    println!("Part 1: {:?}", part1()?);
    println!("Part 2: {:?}", part2()?);

    Ok(())
}

const INPUT_PATH: &str = "inputs/day2.txt";

fn part1() -> Result<u32, AocError> {
    let moves = parse_moves()?;
    let scores: Vec<_> = moves.into_iter().map(round_score).collect();
    let total_score = scores.into_iter().sum();

    Ok(total_score)
}

fn parse_line(line: String) -> Result<(Move, Move), AocError> {
    let v: Vec<_> = line.split(' ').collect();
    match v.as_slice() {
        [opponent, our] => Ok((parse_opponent_move(opponent)?, parse_our_move(our)?)),
        _ => Err(InputError::InvalidValue.into()),
    }
}

fn parse_moves() -> Result<Vec<(Move, Move)>, AocError> {
    let lines = read_lines(INPUT_PATH)?;

    lines.into_iter().map(parse_line).collect()
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn get_outcome((opponent, our): (Move, Move)) -> Outcome {
    use Move::*;
    use Outcome::*;

    match (opponent, our) {
        (Rock, Paper) => Win,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Rock, Rock) => Draw,
        (Paper, Paper) => Draw,
        (Scissors, Scissors) => Draw,
        (Rock, Scissors) => Lose,
        (Paper, Rock) => Lose,
        (Scissors, Paper) => Lose,
    }
}

fn round_score((opponent, our): (Move, Move)) -> u32 {
    let selected_score: u32 = match our {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let outcome = get_outcome((opponent, our));

    let outcome_score: u32 = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };

    selected_score + outcome_score
}

// Part 2

fn part2() -> Result<u32, AocError> {
    let moves_outcomes = parse_moves_outcomes()?;
    let moves: Vec<(Move, Move)> = moves_outcomes
        .into_iter()
        .map(|(opponent, outcome)| (opponent.clone(), get_move_for_outcome(opponent, outcome)))
        .collect();

    let scores: Vec<_> = moves.into_iter().map(round_score).collect();
    let total_score = scores.into_iter().sum();

    Ok(total_score)
}

impl FromStr for Move {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(Self::Err::InvalidValue),
        }
    }
}

impl FromStr for Outcome {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(Self::Err::InvalidValue),
        }
    }
}

fn parse_move_outcome_line(line: String) -> Result<(Move, Outcome), AocError> {
    let v: Vec<_> = line.split(' ').collect();
    match v.as_slice() {
        [opponent, outcome] => Ok((opponent.parse()?, outcome.parse()?)),
        _ => Err(InputError::InvalidValue.into()),
    }
}

fn parse_moves_outcomes() -> Result<Vec<(Move, Outcome)>, AocError> {
    let lines = read_lines(INPUT_PATH)?;

    lines.into_iter().map(parse_move_outcome_line).collect()
}

fn get_move_for_outcome(opponent: Move, outcome: Outcome) -> Move {
    use Move::*;
    use Outcome::*;

    match outcome {
        Win => match opponent {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        Draw => opponent,
        Lose => match opponent {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
    }
}
