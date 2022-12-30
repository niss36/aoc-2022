use std::{collections::HashMap, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day14Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidPoint,
    NotEnoughPoints,
    InvalidRockPart,
    InvalidBoundaries,
}

impl From<io::Error> for Day14Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day14Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day14.txt";

fn main() -> Result<(), Day14Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = Day14Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(",").collect();
        match v.as_slice() {
            [x, y] => Ok(Point(x.parse()?, y.parse()?)),
            _ => Err(Self::Err::InvalidPoint),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RockStructure {
    points: Vec<Point>,
}

impl FromStr for RockStructure {
    type Err = Day14Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s
            .split(" -> ")
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        if points.len() > 1 {
            Ok(Self { points })
        } else {
            Err(Self::Err::NotEnoughPoints)
        }
    }
}

impl RockStructure {
    fn rock_points(&self) -> Result<Vec<Point>, Day14Error> {
        let mut points = self.points.iter();
        let mut prev_point = points.next().expect("points should not be empty");

        let mut result = vec![];

        fn add_segment(
            &Point(start_x, start_y): &Point,
            &Point(end_x, end_y): &Point,
            result: &mut Vec<Point>,
        ) -> Result<(), Day14Error> {
            if start_x == end_x {
                if start_y < end_y {
                    for y in start_y..end_y {
                        result.push(Point(start_x, y));
                    }
                } else {
                    for y in (end_y + 1..=start_y).rev() {
                        result.push(Point(start_x, y));
                    }
                }
                Ok(())
            } else if start_y == end_y {
                if start_x < end_x {
                    for x in start_x..end_x {
                        result.push(Point(x, start_y));
                    }
                } else {
                    for x in (end_x + 1..=start_x).rev() {
                        result.push(Point(x, start_y));
                    }
                }
                Ok(())
            } else {
                Err(Day14Error::InvalidRockPart)
            }
        }

        for point in points {
            add_segment(prev_point, point, &mut result)?;

            prev_point = point;
        }

        result.push(prev_point.clone());

        Ok(result)
    }
}

fn parse_rock_structures(input: &Vec<String>) -> Result<Vec<RockStructure>, Day14Error> {
    input.iter().map(|line| line.parse()).collect()
}

#[derive(Debug)]
struct Boundaries {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

fn find_boundaries(rock_structures: &Vec<RockStructure>) -> Boundaries {
    let top = 0;
    let mut right = 500;
    let mut bottom = 0;
    let mut left = 500;

    for structure in rock_structures {
        for Point(x, y) in &structure.points {
            right = right.max(*x);
            bottom = bottom.max(*y);
            left = left.min(*x);
        }
    }

    Boundaries {
        left,
        top,
        right,
        bottom,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileContents {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Eq)]
struct Cave {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    contents: Vec<TileContents>,
}

impl Cave {
    fn new(
        Boundaries {
            left,
            top,
            right,
            bottom,
        }: Boundaries,
    ) -> Result<Self, Day14Error> {
        if left <= right && top <= bottom {
            let width = right - left + 1;
            let height = bottom - top + 1;

            Ok(Self {
                left,
                top,
                width,
                height,
                contents: vec![TileContents::Air; width * height],
            })
        } else {
            Err(Day14Error::InvalidBoundaries)
        }
    }

    fn get_index(&self, Point(x, y): &Point) -> Option<usize> {
        if x >= &self.left && y >= &self.top {
            let x = x - self.left;
            let y = y - self.top;

            Some(x + y * self.width)
        } else {
            None
        }
    }

    fn get(&self, point: &Point) -> Option<&TileContents> {
        let index = self.get_index(point)?;

        self.contents.get(index)
    }

    fn get_mut(&mut self, point: &Point) -> Option<&mut TileContents> {
        let index = self.get_index(point)?;

        self.contents.get_mut(index)
    }

    fn simulate_sand(&mut self) -> bool {
        use TileContents::*;

        let mut x = 500;
        let mut y = 0;

        loop {
            match self.get(&Point(x, y + 1)) {
                Some(Air) => {
                    y = y + 1;
                }
                Some(_) => match self.get(&Point(x - 1, y + 1)) {
                    Some(Air) => {
                        x = x - 1;
                        y = y + 1;
                    }
                    Some(_) => match self.get(&Point(x + 1, y + 1)) {
                        Some(Air) => {
                            x = x + 1;
                            y = y + 1;
                        }
                        Some(_) => match self.get_mut(&Point(x, y)) {
                            Some(tile) => {
                                debug_assert_eq!(*tile, Air);

                                *tile = Sand;
                                return true;
                            }
                            None => return false,
                        },
                        None => return false,
                    },
                    None => return false,
                },
                None => return false,
            }
        }
    }
}

fn parse_cave(input: &Vec<String>) -> Result<Cave, Day14Error> {
    let rock_structures = parse_rock_structures(input)?;

    let boundaries = find_boundaries(&rock_structures);
    let mut cave = Cave::new(boundaries)?;

    for rock_structure in rock_structures {
        for point in rock_structure.rock_points()? {
            if let Some(tile) = cave.get_mut(&point) {
                *tile = TileContents::Rock;
            }
        }
    }

    Ok(cave)
}

fn part1(input: &Vec<String>) -> Result<usize, Day14Error> {
    let mut cave = parse_cave(input)?;

    let mut sand_count = 0;
    while cave.simulate_sand() {
        sand_count += 1;
    }

    Ok(sand_count)
}

struct Cave2 {
    height: usize,
    contents: HashMap<Point, TileContents>,
}

impl Cave2 {
    fn new(
        Boundaries {
            left,
            top,
            right,
            bottom,
        }: Boundaries,
    ) -> Result<Self, Day14Error> {
        if left <= right && top <= bottom {
            let height = bottom - top + 1;

            Ok(Self {
                height,
                contents: HashMap::new(),
            })
        } else {
            Err(Day14Error::InvalidBoundaries)
        }
    }

    fn get(&self, point: &Point) -> &TileContents {
        if point.1 == self.height + 1 {
            &TileContents::Rock
        } else {
            self.contents.get(point).unwrap_or(&TileContents::Air)
        }
    }

    fn set(&mut self, point: Point, value: TileContents) {
        self.contents.insert(point, value);
    }

    fn simulate_sand(&mut self) -> bool {
        use TileContents::*;

        let mut x = 500;
        let mut y = 0;

        loop {
            match self.get(&Point(x, y + 1)) {
                Air => {
                    y = y + 1;
                }
                _ => match self.get(&Point(x - 1, y + 1)) {
                    Air => {
                        x = x - 1;
                        y = y + 1;
                    }
                    _ => match self.get(&Point(x + 1, y + 1)) {
                        Air => {
                            x = x + 1;
                            y = y + 1;
                        }
                        _ => match self.get(&Point(x, y)) {
                            Air => {
                                self.set(Point(x, y), Sand);
                                return true;
                            }
                            _ => return false,
                        },
                    },
                },
            }
        }
    }
}

fn parse_cave_2(input: &Vec<String>) -> Result<Cave2, Day14Error> {
    let rock_structures = parse_rock_structures(input)?;

    let boundaries = find_boundaries(&rock_structures);
    let mut cave = Cave2::new(boundaries)?;

    for rock_structure in rock_structures {
        for point in rock_structure.rock_points()? {
            cave.set(point, TileContents::Rock);
        }
    }

    Ok(cave)
}

fn part2(input: &Vec<String>) -> Result<usize, Day14Error> {
    let mut cave = parse_cave_2(input)?;

    let mut sand_count = 0;
    while cave.simulate_sand() {
        sand_count += 1;
    }

    Ok(sand_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_parse_rock_structure() {
        let value: RockStructure = "498,4 -> 498,6 -> 496,6".parse().unwrap();
        let expected = RockStructure {
            points: vec![Point(498, 4), Point(498, 6), Point(496, 6)],
        };

        assert_eq!(value, expected);
    }

    #[test]
    fn test_rock_structure_rock_points() {
        let value: RockStructure = "498,4 -> 498,6 -> 496,6".parse().unwrap();

        assert_eq!(
            value.rock_points().unwrap(),
            vec![
                Point(498, 4),
                Point(498, 5),
                Point(498, 6),
                Point(497, 6),
                Point(496, 6)
            ]
        )
    }

    #[test]
    fn test_parse_cave() {
        use TileContents::*;

        let input = to_lines(EXAMPLE);

        let expected = Cave {
            left: 494,
            top: 0,
            width: 10,
            height: 10,
            contents: vec![
                Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
                Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
                Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Rock, Air, Air, Air,
                Rock, Rock, Air, Air, Air, Air, Rock, Air, Air, Air, Rock, Air, Air, Air, Rock,
                Rock, Rock, Air, Air, Air, Rock, Air, Air, Air, Air, Air, Air, Air, Air, Air, Rock,
                Air, Air, Air, Air, Air, Air, Air, Air, Air, Rock, Air, Rock, Rock, Rock, Rock,
                Rock, Rock, Rock, Rock, Rock, Air,
            ],
        };

        assert_eq!(parse_cave(&input).unwrap(), expected);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 24);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 93);
    }
}
