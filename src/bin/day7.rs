use std::{collections::HashMap, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day7Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidFileSystemItem(String),
    InvalidCommand(String),
    NotADirectory,
    ItemNotFound,
    NoSolution,
}

impl From<io::Error> for Day7Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day7Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day7.txt";

fn main() -> Result<(), Day7Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

fn parse_directory_entry(s: &str) -> Result<(String, File), Day7Error> {
    let v: Vec<_> = s.split(" ").collect();
    match v.as_slice() {
        ["dir", name] => Ok((name.to_string(), File::Directory(HashMap::new()))),
        [size, name] => Ok((name.to_string(), File::File(size.parse()?))),
        _ => Err(Day7Error::InvalidFileSystemItem(s.to_string())),
    }
}

#[derive(Debug)]
enum File {
    File(usize),
    Directory(HashMap<String, File>),
}

#[derive(Debug)]
enum Command {
    CdRoot,
    CdParent,
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = Day7Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(" ").collect();
        match v.as_slice() {
            ["$", "cd", "/"] => Ok(Self::CdRoot),
            ["$", "cd", ".."] => Ok(Self::CdParent),
            ["$", "cd", name] => Ok(Self::Cd(name.to_string())),
            ["$", "ls"] => Ok(Self::Ls),
            _ => Err(Self::Err::InvalidCommand(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct CommandOutputPair {
    command: Command,
    output: Vec<String>,
}

impl TryFrom<Vec<String>> for CommandOutputPair {
    type Error = Day7Error;

    fn try_from(mut value: Vec<String>) -> Result<Self, Self::Error> {
        let command = value.remove(0);

        Ok(Self {
            command: command.parse()?,
            output: value,
        })
    }
}

fn parse_command_output_pairs(input: &Vec<String>) -> Result<Vec<CommandOutputPair>, Day7Error> {
    let mut accumulator: Vec<String> = vec![];
    let mut command_output_pairs: Vec<CommandOutputPair> = vec![];

    for line in input {
        if !accumulator.is_empty() && line.starts_with("$") {
            command_output_pairs.push(accumulator.try_into()?);
            accumulator = vec![];
        }
        accumulator.push(line.to_string());
    }

    command_output_pairs.push(accumulator.try_into()?);

    Ok(command_output_pairs)
}

struct State {
    root: File,
    path: Vec<String>,
}

fn find_item<'a>(root: &'a mut File, path: &[String]) -> Result<&'a mut File, Day7Error> {
    match path {
        [] => Ok(root),
        [component, rest @ ..] => match root {
            File::File(_) => Err(Day7Error::NotADirectory),
            File::Directory(items) => {
                let file = items.get_mut(component).ok_or(Day7Error::ItemNotFound)?;

                find_item(file, rest)
            }
        },
    }
}

fn reduce(
    mut state: State,
    CommandOutputPair { command, output }: CommandOutputPair,
) -> Result<State, Day7Error> {
    match command {
        Command::CdRoot => Ok(State {
            root: state.root,
            path: vec![],
        }),
        Command::CdParent => {
            state.path.pop();
            Ok(state)
        }
        Command::Cd(name) => {
            state.path.push(name);
            Ok(state)
        }
        Command::Ls => {
            let entries = output
                .iter()
                .map(|line| parse_directory_entry(line))
                .collect::<Result<HashMap<String, File>, Day7Error>>()?;

            if let File::Directory(e) = find_item(&mut state.root, &state.path)? {
                *e = entries;
                Ok(state)
            } else {
                Err(Day7Error::NotADirectory)
            }
        }
    }
}

fn infer_structure<'a>(command_output_pairs: Vec<CommandOutputPair>) -> Result<File, Day7Error> {
    let mut state = State {
        root: File::Directory(HashMap::new()),
        path: vec![],
    };

    for command_output_pair in command_output_pairs {
        state = reduce(state, command_output_pair)?;
    }

    Ok(state.root)
}

fn total_size(file: &File) -> usize {
    match file {
        File::File(size) => size.to_owned(),
        File::Directory(entries) => entries.values().map(total_size).sum(),
    }
}

#[derive(Debug)]
struct Walk<'a> {
    to_explore: Vec<(&'a String, &'a File)>,
}

impl<'a> Walk<'a> {
    fn new(root: (&'a String, &'a File)) -> Walk<'a> {
        Walk {
            to_explore: vec![root],
        }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = (&'a String, &'a File);

    fn next(&mut self) -> Option<Self::Item> {
        self.to_explore.pop().map(|(name, file)| {
            if let File::Directory(entries) = file {
                self.to_explore.extend(entries);
            }

            (name, file)
        })
    }
}

fn part1(input: &Vec<String>) -> Result<usize, Day7Error> {
    let command_output_pairs = parse_command_output_pairs(input)?;
    let root = infer_structure(command_output_pairs)?;

    let total = Walk::new((&String::from("/"), &root))
        .filter_map(|(_, file)| match file {
            File::File(_) => None,
            File::Directory(_) => Some(total_size(file)),
        })
        .filter(|size| size <= &100000)
        .sum();

    Ok(total)
}

fn part2(input: &Vec<String>) -> Result<usize, Day7Error> {
    let command_output_pairs = parse_command_output_pairs(input)?;
    let root = infer_structure(command_output_pairs)?;

    let unused_space = 70000000 - total_size(&root);
    let required_space = 30000000 - unused_space;

    Walk::new((&String::from("/"), &root))
        .filter_map(|(_, file)| match file {
            File::File(_) => None,
            File::Directory(_) => Some(total_size(file)),
        })
        .filter(|size| size >= &required_space)
        .min()
        .ok_or(Day7Error::NoSolution)
}
