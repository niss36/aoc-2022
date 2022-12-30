use std::{
    collections::{HashMap, HashSet},
    io,
};

use aoc::read_lines;

#[derive(Debug)]
enum Day17Error {
    IoError(io::Error),
    InvalidJetError(char),
    EmptyInput,
    EmptyJetPattern,
}

impl From<io::Error> for Day17Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day17.txt";

fn main() -> Result<(), Day17Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl TryFrom<char> for Jet {
    type Error = Day17Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(Self::Error::InvalidJetError(value)),
        }
    }
}

fn parse_jet_pattern(input: &Vec<String>) -> Result<Vec<Jet>, Day17Error> {
    let input = input.first().ok_or(Day17Error::EmptyInput)?;

    input.chars().map(|c| c.try_into()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy)]
enum RockShape {
    Horizontal,
    Plus,
    Wedge,
    Vertical,
    Square,
}

const ROCK_SHAPES: [RockShape; 5] = [
    RockShape::Horizontal,
    RockShape::Plus,
    RockShape::Wedge,
    RockShape::Vertical,
    RockShape::Square,
];

impl RockShape {
    fn points(&self, bottom_left: &Point) -> Vec<Point> {
        use RockShape::*;
        let Point { x, y } = bottom_left;

        match self {
            Horizontal => vec![
                Point { x: *x, y: *y },
                Point { x: *x + 1, y: *y },
                Point { x: *x + 2, y: *y },
                Point { x: *x + 3, y: *y },
            ],
            Plus => vec![
                Point { x: *x + 1, y: *y },
                Point { x: *x, y: *y + 1 },
                Point {
                    x: *x + 1,
                    y: *y + 1,
                },
                Point {
                    x: *x + 2,
                    y: *y + 1,
                },
                Point {
                    x: *x + 1,
                    y: *y + 2,
                },
            ],
            Wedge => vec![
                Point { x: *x, y: *y },
                Point { x: *x + 1, y: *y },
                Point { x: *x + 2, y: *y },
                Point {
                    x: *x + 2,
                    y: *y + 1,
                },
                Point {
                    x: *x + 2,
                    y: *y + 2,
                },
            ],
            Vertical => vec![
                Point { x: *x, y: *y },
                Point { x: *x, y: *y + 1 },
                Point { x: *x, y: *y + 2 },
                Point { x: *x, y: *y + 3 },
            ],
            Square => vec![
                Point { x: *x, y: *y },
                Point { x: *x + 1, y: *y },
                Point { x: *x, y: *y + 1 },
                Point {
                    x: *x + 1,
                    y: *y + 1,
                },
            ],
        }
    }
}

struct CaveState {
    shape_index: usize,
    jet_index: usize,
    jet_pattern: Vec<Jet>,
    fallen_rocks: HashSet<Point>,
    heights: [u64; 7],
}

impl CaveState {
    fn new(jet_pattern: Vec<Jet>) -> Result<Self, Day17Error> {
        if jet_pattern.len() == 0 {
            Err(Day17Error::EmptyJetPattern)
        } else {
            Ok(Self {
                shape_index: 0,
                jet_index: 0,
                jet_pattern,
                fallen_rocks: HashSet::new(),
                heights: [0; 7],
            })
        }
    }

    fn height(&self) -> u64 {
        self.heights.into_iter().max().unwrap()
    }

    fn next_shape(&mut self) -> RockShape {
        let shape = ROCK_SHAPES[self.shape_index];
        self.shape_index = (self.shape_index + 1) % ROCK_SHAPES.len();

        shape
    }

    fn next_jet(&mut self) -> Jet {
        let jet = self.jet_pattern[self.jet_index];
        self.jet_index = (self.jet_index + 1) % self.jet_pattern.len();

        jet
    }

    fn drop_rock(&mut self) -> Result<(), Day17Error> {
        use Jet::*;
        // Floor is y = 0
        // Left wall is x = 0
        // Right wall is x = 8

        fn collides(points: Vec<Point>, fallen_rocks: &HashSet<Point>) -> bool {
            points.iter().any(|point| fallen_rocks.contains(point))
        }

        let shape = self.next_shape();
        let mut bottom_left = Point {
            x: 3,
            y: self.height() + 4,
        };

        loop {
            match self.next_jet() {
                Left => {
                    if bottom_left.x > 1 {
                        let new_bottom_left = Point {
                            x: bottom_left.x - 1,
                            y: bottom_left.y,
                        };

                        if !collides(shape.points(&new_bottom_left), &self.fallen_rocks) {
                            bottom_left = new_bottom_left;
                        }
                    }
                }
                Right => {
                    let new_bottom_left = Point {
                        x: bottom_left.x + 1,
                        y: bottom_left.y,
                    };

                    let points = shape.points(&new_bottom_left);

                    if points.iter().all(|point| point.x <= 7)
                        && !collides(points, &self.fallen_rocks)
                    {
                        bottom_left = new_bottom_left;
                    }
                }
            };

            if bottom_left.y > 1 {
                let new_bottom_left = Point {
                    x: bottom_left.x,
                    y: bottom_left.y - 1,
                };

                if !collides(shape.points(&new_bottom_left), &self.fallen_rocks) {
                    bottom_left = new_bottom_left;
                    continue;
                }
            }

            let points = shape.points(&bottom_left);
            for point in &points {
                if point.y > self.heights[point.x as usize - 1] {
                    self.heights[point.x as usize - 1] = point.y;
                }
            }

            self.fallen_rocks.extend(points);
            break;
        }

        Ok(())
    }

    fn surface(&self) -> [u64; 7] {
        let min = self.heights.into_iter().min().unwrap();

        self.heights.map(|height| height - min)
    }
}

fn part1(input: &Vec<String>) -> Result<u64, Day17Error> {
    let jet_pattern = parse_jet_pattern(input)?;
    let mut cave_state = CaveState::new(jet_pattern)?;

    for _ in 0..2022 {
        cave_state.drop_rock()?;
    }

    Ok(cave_state.height())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CycleState {
    surface: [u64; 7],
    shape_index: usize,
    jet_index: usize,
}

fn part2(input: &Vec<String>) -> Result<u64, Day17Error> {
    let jet_pattern = parse_jet_pattern(input)?;
    let mut cave_state = CaveState::new(jet_pattern)?;

    // Map each state to the index where it was encountered
    let mut cycle_memory: HashMap<CycleState, usize> = HashMap::new();
    let mut height_memory: Vec<u64> = vec![];

    let n: usize = 1000000000000;

    for i in 0..n {
        let cycle_state = CycleState {
            surface: cave_state.surface(),
            shape_index: cave_state.shape_index,
            jet_index: cave_state.jet_index,
        };
        if let Some(previous_i) = cycle_memory.get(&cycle_state) {
            let cycle_length = i - previous_i;

            let current_height = cave_state.height();
            let previous_height = height_memory[*previous_i];
            let height_gain_per_cycle = current_height - previous_height;

            let number_of_cycles = (n - i) / cycle_length;
            let remainder = (n - i) % cycle_length;

            let intermediate_index = previous_i + remainder;
            let intermediate_height = height_memory[intermediate_index];
            let intermediate_height_gain = intermediate_height - previous_height;

            let total_height = current_height
                + height_gain_per_cycle * (number_of_cycles as u64)
                + intermediate_height_gain;

            return Ok(total_height);
        }

        height_memory.push(cave_state.height());
        cycle_memory.insert(cycle_state, i);

        cave_state.drop_rock()?;
    }

    Ok(cave_state.height())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part1(&input).unwrap(), 3068);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part2(&input).unwrap(), 1514285714288);
    }
}
