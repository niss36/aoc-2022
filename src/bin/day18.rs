use std::{collections::HashSet, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day18Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidCube,
    EmptyInput,
}

impl From<io::Error> for Day18Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day18Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day18.txt";

fn main() -> Result<(), Day18Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: u64,
    y: u64,
    z: u64,
}

impl Cube {
    fn neighbours(&self) -> Vec<Self> {
        let &Cube { x, y, z } = self;

        let mut neighbours = vec![
            Self { x: x + 1, y, z },
            Self { x, y: y + 1, z },
            Self { x, y, z: z + 1 },
        ];

        if x > 0 {
            neighbours.push(Self { x: x - 1, y, z });
        }

        if y > 0 {
            neighbours.push(Self { x, y: y - 1, z });
        }

        if z > 0 {
            neighbours.push(Self { x, y, z: z - 1 });
        }

        neighbours
    }

    fn number_of_exposed_sides(&self, others: &HashSet<Self>) -> usize {
        let neighbours = self.neighbours();

        (6 - neighbours.len())
            + neighbours
                .into_iter()
                .filter(|neighbour| !others.contains(neighbour))
                .count()
    }

    fn number_of_exposed_sides_2(&self, exterior: &HashSet<Self>) -> usize {
        let neighbours = self.neighbours();

        (6 - neighbours.len())
            + neighbours
                .into_iter()
                .filter(|neighbour| exterior.contains(neighbour))
                .count()
    }
}

impl FromStr for Cube {
    type Err = Day18Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(",").collect();
        match v.as_slice() {
            [x, y, z] => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
                z: z.parse()?,
            }),
            _ => Err(Self::Err::InvalidCube),
        }
    }
}

fn parse_cubes(input: &Vec<String>) -> Result<HashSet<Cube>, Day18Error> {
    input.iter().map(|line| line.parse()).collect()
}

fn part1(input: &Vec<String>) -> Result<usize, Day18Error> {
    let cubes = parse_cubes(input)?;

    Ok(cubes
        .iter()
        .map(|cube| cube.number_of_exposed_sides(&cubes))
        .sum())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Boundaries {
    min_x: u64,
    min_y: u64,
    min_z: u64,
    max_x: u64,
    max_y: u64,
    max_z: u64,
}

impl From<&Cube> for Boundaries {
    fn from(cube: &Cube) -> Self {
        let &Cube { x, y, z } = cube;

        Self {
            min_x: x,
            min_y: y,
            min_z: z,
            max_x: x,
            max_y: y,
            max_z: z,
        }
    }
}

impl Boundaries {
    fn contains(&self, cube: &Cube) -> bool {
        cube.x >= self.min_x
            && cube.x <= self.max_x
            && cube.y >= self.min_y
            && cube.y <= self.max_y
            && cube.z >= self.min_z
            && cube.z <= self.max_z
    }

    fn update(mut self, cube: &Cube) -> Self {
        let &Cube { x, y, z } = cube;

        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);

        self
    }

    fn expand_1(mut self) -> Self {
        self.min_x = self.min_x.saturating_sub(1);
        self.min_y = self.min_y.saturating_sub(1);
        self.min_z = self.min_z.saturating_sub(1);
        self.max_x = self.max_x.saturating_add(1);
        self.max_y = self.max_y.saturating_add(1);
        self.max_z = self.max_z.saturating_add(1);

        self
    }
}

fn compute_boundaries(cubes: &HashSet<Cube>) -> Result<Boundaries, Day18Error> {
    let mut cubes = cubes.iter();

    let first = cubes.next().ok_or(Day18Error::EmptyInput)?;

    Ok(cubes.fold(first.into(), Boundaries::update))
}

fn find_exterior(cubes: &HashSet<Cube>, boundaries: Boundaries) -> HashSet<Cube> {
    // Add a 1 unit gap on all sides to make sure there is a single contiguous exterior
    let boundaries = boundaries.expand_1();

    let start = Cube {
        x: boundaries.min_x,
        y: boundaries.min_y,
        z: boundaries.min_z,
    };

    assert!(!cubes.contains(&start));

    let mut exterior = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = vec![start];

    while let Some(cube) = to_visit.pop() {
        visited.insert(cube);

        if !cubes.contains(&cube) {
            exterior.insert(cube);

            for neighbour in cube.neighbours() {
                if boundaries.contains(&neighbour) && !visited.contains(&neighbour) {
                    to_visit.push(neighbour);
                }
            }
        }
    }

    exterior
}

fn part2(input: &Vec<String>) -> Result<usize, Day18Error> {
    let cubes = parse_cubes(input)?;
    let boundaries = compute_boundaries(&cubes)?;

    let exterior = find_exterior(&cubes, boundaries);

    Ok(cubes
        .iter()
        .map(|cube| cube.number_of_exposed_sides_2(&exterior))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part1(&input).unwrap(), 64);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part2(&input).unwrap(), 58);
    }
}
