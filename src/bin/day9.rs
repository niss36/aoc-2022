use std::{collections::HashSet, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day9Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidDirection,
    InvalidStepFormat,
}

impl From<io::Error> for Day9Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day9Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day9.txt";

fn main() -> Result<(), Day9Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = Day9Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(Self::Err::InvalidDirection),
        }
    }
}

struct Step(Direction, usize);

impl FromStr for Step {
    type Err = Day9Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(" ").collect();
        match v.as_slice() {
            [direction, number] => Ok(Step(direction.parse()?, number.parse()?)),
            _ => Err(Self::Err::InvalidStepFormat),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Displacement {
    Center,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl From<Direction> for Displacement {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        use Displacement::*;

        match direction {
            Up => North,
            Right => East,
            Down => South,
            Left => West,
        }
    }
}

impl Into<(isize, isize)> for Displacement {
    fn into(self) -> (isize, isize) {
        use Displacement::*;

        match self {
            Center => (0, 0),
            North => (0, 1),
            NorthEast => (1, 1),
            East => (1, 0),
            SouthEast => (1, -1),
            South => (0, -1),
            SouthWest => (-1, -1),
            West => (-1, 0),
            NorthWest => (-1, 1),
        }
    }
}

impl From<(isize, isize)> for Displacement {
    fn from((x, y): (isize, isize)) -> Self {
        use std::cmp::Ordering::*;
        use Displacement::*;

        match x.abs().cmp(&y.abs()) {
            Less => {
                if y > 0 {
                    North
                } else {
                    // |x| < |y| && y == 0 is impossible
                    South
                }
            }
            Equal => {
                if x > 0 {
                    if y > 0 {
                        NorthEast
                    } else {
                        // |x| == |y| && x > 0 ==> y != 0
                        SouthEast
                    }
                } else if x < 0 {
                    if y > 0 {
                        NorthWest
                    } else {
                        // |x| == |y| && x < 0 ==> y != 0
                        SouthWest
                    }
                } else {
                    // |x| == |y| && x == 0 ==> y == 0
                    Center
                }
            }
            Greater => {
                if x > 0 {
                    East
                } else {
                    // |x| > |y| && x == 0 is impossible
                    West
                }
            }
        }
    }
}

impl Displacement {
    fn into_position(self, (x, y): &(isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.into();

        (x + dx, y + dy)
    }
}

#[derive(Clone)]
struct RopeState {
    head_position: (isize, isize),
    tail_displacement: Displacement,
}

fn new_tail_displacement(tail_displacement: Displacement, motion: Displacement) -> Displacement {
    let (x, y) = tail_displacement.into();
    let (dx, dy) = motion.into();

    Displacement::from((x - dx, y - dy))
}

impl RopeState {
    fn new() -> Self {
        Self {
            head_position: (0, 0),
            tail_displacement: Displacement::Center,
        }
    }

    fn tail_position(&self) -> (isize, isize) {
        self.tail_displacement.into_position(&self.head_position)
    }

    fn apply_step(self, step: Direction) -> Self {
        self.apply_motion(step.into())
    }

    fn apply_motion(&self, motion: Displacement) -> Self {
        let head_position = motion.into_position(&self.head_position);
        let tail_displacement = new_tail_displacement(self.tail_displacement, motion);

        Self {
            head_position,
            tail_displacement,
        }
    }
}

fn parse_steps(input: &Vec<String>) -> Result<Vec<Step>, Day9Error> {
    input.iter().map(|line| line.parse()).collect()
}

fn part1(input: &Vec<String>) -> Result<usize, Day9Error> {
    let steps = parse_steps(input)?;

    let mut rope_state = RopeState::new();
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    tail_positions.insert(rope_state.tail_position());

    for Step(direction, number) in steps {
        for _ in 0..number {
            rope_state = rope_state.apply_step(direction);
            tail_positions.insert(rope_state.tail_position());
        }
    }

    Ok(tail_positions.len())
}

#[derive(Clone)]
struct ExtendedRopeState {
    head_position: (isize, isize),
    knot_displacements: Vec<Displacement>,
}

impl ExtendedRopeState {
    fn new(n_knots: usize) -> Self {
        Self {
            head_position: (0, 0),
            knot_displacements: vec![Displacement::Center; n_knots],
        }
    }

    fn tail_position(&self) -> (isize, isize) {
        self.knot_displacements
            .iter()
            .fold(self.head_position, |position, displacement| {
                displacement.into_position(&position)
            })
    }

    fn apply_motion(mut self, mut motion: Displacement) -> Self {
        let mut prev_anchor_position = self.head_position;
        let mut new_anchor_position = motion.into_position(&prev_anchor_position);
        self.head_position = new_anchor_position;

        for displacement in self.knot_displacements.iter_mut() {
            prev_anchor_position = displacement.into_position(&prev_anchor_position);
            *displacement = new_tail_displacement(*displacement, motion);
            new_anchor_position = displacement.into_position(&new_anchor_position);

            let (prev_x, prev_y) = prev_anchor_position;
            let (new_x, new_y) = new_anchor_position;

            motion = Displacement::from((new_x - prev_x, new_y - prev_y));
        }

        self
    }
}

fn part2(input: &Vec<String>) -> Result<usize, Day9Error> {
    let steps = parse_steps(input)?;

    let mut rope_state = ExtendedRopeState::new(9);
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    tail_positions.insert(rope_state.tail_position());

    for Step(direction, number) in steps {
        for _ in 0..number {
            rope_state = rope_state.apply_motion(direction.into());
            tail_positions.insert(rope_state.tail_position());
        }
    }

    Ok(tail_positions.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Displacement::*;

    const DISPLACEMENTS: [Displacement; 9] = [
        Center, North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
    ];

    #[test]
    fn test_displacement_from_into_eq() {
        for displacement in DISPLACEMENTS {
            let (x, y) = displacement.into();
            assert_eq!(Displacement::from((x, y)), displacement);
        }
    }

    #[test]
    fn test_displacement_from_twice_into_eq() {
        for displacement in DISPLACEMENTS {
            let (x, y) = displacement.into();
            assert_eq!(Displacement::from((2 * x, 2 * y)), displacement);
        }
    }

    #[test]
    fn test_displacement_from() {
        assert_eq!(Displacement::from((1, 2)), North);
        assert_eq!(Displacement::from((-1, 2)), North);

        assert_eq!(Displacement::from((-1, -2)), South);
        assert_eq!(Displacement::from((1, -2)), South);

        assert_eq!(Displacement::from((2, 1)), East);
        assert_eq!(Displacement::from((2, -1)), East);

        assert_eq!(Displacement::from((-2, -1)), West);
        assert_eq!(Displacement::from((-2, 1)), West);
    }

    #[test]
    fn test_extended_rope_1_knot_tail_position() {
        let rope_state = RopeState::new();
        let extended_rope_state = ExtendedRopeState::new(1);

        assert_eq!(
            rope_state.tail_position(),
            extended_rope_state.tail_position()
        );
    }

    #[test]
    fn test_extended_rope_1_knot_apply_motion() {
        let rope_state = RopeState::new();
        let extended_rope_state = ExtendedRopeState::new(1);

        for motion in DISPLACEMENTS {
            let new_rope_state = rope_state.apply_motion(motion);
            let new_extended_rope_state = extended_rope_state.clone().apply_motion(motion);

            assert_eq!(
                new_rope_state.tail_position(),
                new_extended_rope_state.tail_position()
            );
        }
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(part1(&input).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"
        .lines()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(part2(&input).unwrap(), 36);
    }
}
