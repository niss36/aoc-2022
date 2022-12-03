use std::{io, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day2Error {
    IoError(io::Error),
    InvalidMove(String),
    InvalidOutcome(String),
    InvalidFormat(String),
}

impl From<io::Error> for Day2Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day2.txt";

fn main() -> Result<(), Day2Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse_their_move(s: &str) -> Result<Move, Day2Error> {
    match s {
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        _ => Err(Day2Error::InvalidMove(s.to_string())),
    }
}

fn parse_our_move(s: &str) -> Result<Move, Day2Error> {
    match s {
        "X" => Ok(Move::Rock),
        "Y" => Ok(Move::Paper),
        "Z" => Ok(Move::Scissors),
        _ => Err(Day2Error::InvalidMove(s.to_string())),
    }
}

fn part1(input: &Vec<String>) -> Result<u32, Day2Error> {
    let moves = parse_moves(input)?;
    let scores: Vec<_> = moves.into_iter().map(round_score).collect();
    let total_score = scores.into_iter().sum();

    Ok(total_score)
}

fn parse_line(line: &String) -> Result<(Move, Move), Day2Error> {
    let v: Vec<_> = line.split(' ').collect();
    match v.as_slice() {
        [opponent, our] => Ok((parse_their_move(opponent)?, parse_our_move(our)?)),
        _ => Err(Day2Error::InvalidFormat(line.clone())),
    }
}

fn parse_moves(lines: &Vec<String>) -> Result<Vec<(Move, Move)>, Day2Error> {
    lines.into_iter().map(parse_line).collect()
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn get_outcome((their_move, our_move): (Move, Move)) -> Outcome {
    use Move::*;
    use Outcome::*;

    match (their_move, our_move) {
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

fn round_score((their_move, our_move): (Move, Move)) -> u32 {
    use Move::*;
    use Outcome::*;

    let selected_score: u32 = match our_move {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let outcome = get_outcome((their_move, our_move));

    let outcome_score: u32 = match outcome {
        Win => 6,
        Draw => 3,
        Lose => 0,
    };

    selected_score + outcome_score
}

// Part 2

fn part2(input: &Vec<String>) -> Result<u32, Day2Error> {
    let moves_outcomes = parse_moves_outcomes(input)?;
    let moves: Vec<(Move, Move)> = moves_outcomes
        .into_iter()
        .map(|(their_move, outcome)| {
            (
                their_move.clone(),
                get_move_for_outcome(their_move, outcome),
            )
        })
        .collect();

    let scores: Vec<_> = moves.into_iter().map(round_score).collect();
    let total_score = scores.into_iter().sum();

    Ok(total_score)
}

impl FromStr for Move {
    type Err = Day2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(Self::Err::InvalidMove(s.to_string())),
        }
    }
}

impl FromStr for Outcome {
    type Err = Day2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(Self::Err::InvalidOutcome(s.to_string())),
        }
    }
}

fn parse_move_outcome_line(line: &String) -> Result<(Move, Outcome), Day2Error> {
    let v: Vec<_> = line.split(' ').collect();
    match v.as_slice() {
        [opponent, outcome] => Ok((opponent.parse()?, outcome.parse()?)),
        _ => Err(Day2Error::InvalidFormat(line.clone())),
    }
}

fn parse_moves_outcomes(input: &Vec<String>) -> Result<Vec<(Move, Outcome)>, Day2Error> {
    input.iter().map(parse_move_outcome_line).collect()
}

fn get_move_for_outcome(their_move: Move, outcome: Outcome) -> Move {
    use Move::*;
    use Outcome::*;

    match outcome {
        Win => match their_move {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        Draw => their_move,
        Lose => match their_move {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
    }
}
