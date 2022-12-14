use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

use aoc::read_lines;

#[derive(Debug)]
enum Day12Error {
    IoError(io::Error),
    InconsistentRowWidth,
    NoStartPosition,
    NoEndPosition,
    NoPath,
}

impl From<io::Error> for Day12Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day12.txt";

fn main() -> Result<(), Day12Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

struct ElevationMap {
    width: usize,
    height: usize,
    storage: Vec<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

impl TryFrom<&[String]> for ElevationMap {
    type Error = Day12Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let mut storage: Vec<u8> = vec![];

        let mut width: Option<usize> = None;
        let height = value.len();

        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

        for (y, row) in value.iter().enumerate() {
            let row_width = row.len();
            for (x, elevation) in row.bytes().enumerate() {
                let elevation = match elevation {
                    b'S' => {
                        start = Some((x, y));
                        b'a'
                    }
                    b'E' => {
                        end = Some((x, y));
                        b'z'
                    }
                    e => e,
                };

                storage.push(elevation);
            }

            match width {
                None => {
                    width = Some(row_width);
                }
                Some(width) if width != row_width => return Err(Day12Error::InconsistentRowWidth),
                _ => {}
            }
        }

        let width = width.unwrap_or(0);

        debug_assert!(storage.len() == width * height);

        Ok(Self {
            storage,
            width,
            height,
            start: start.ok_or(Self::Error::NoStartPosition)?,
            end: end.ok_or(Self::Error::NoEndPosition)?,
        })
    }
}

struct PointWithTentativeDistance {
    point: (usize, usize),
    tentative_distance: usize,
}

impl PartialEq for PointWithTentativeDistance {
    fn eq(&self, other: &Self) -> bool {
        self.tentative_distance == other.tentative_distance
    }
}

impl Eq for PointWithTentativeDistance {}

impl PartialOrd for PointWithTentativeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other
            .tentative_distance
            .partial_cmp(&self.tentative_distance)
    }
}

impl Ord for PointWithTentativeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.tentative_distance.cmp(&self.tentative_distance)
    }
}

impl ElevationMap {
    fn index_of(&self, col_index: usize, row_index: usize) -> usize {
        row_index * self.width + col_index
    }

    fn get(&self, col_index: usize, row_index: usize) -> Option<&u8> {
        self.storage.get(self.index_of(col_index, row_index))
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];

        if x > 0 {
            result.push((x - 1, y));
        }

        if y > 0 {
            result.push((x, y - 1));
        }

        if x + 1 < self.width {
            result.push((x + 1, y));
        }

        if y + 1 < self.height {
            result.push((x, y + 1));
        }

        result
    }

    fn neighbours_with_distances(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), usize)> + '_ {
        let current_elevation = self.get(x, y);

        self.neighbours((x, y))
            .into_iter()
            .filter_map(move |(x, y)| {
                let target_elevation = self.get(x, y);

                if target_elevation? <= &(current_elevation? + 1) {
                    Some(((x, y), 1))
                } else {
                    None
                }
            })
    }

    fn length_of_shortest_path(&self, start: (usize, usize)) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        queue.push(PointWithTentativeDistance {
            point: start,
            tentative_distance: 0,
        });

        let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        while let Some(PointWithTentativeDistance {
            point,
            tentative_distance,
        }) = queue.pop()
        {
            if point == self.end {
                let mut length = 0;
                let mut node = point;
                while let Some(prev) = previous.remove(&node) {
                    node = prev;
                    length += 1;
                }

                return Some(length);
            } else {
                for (neighbour, distance) in self.neighbours_with_distances(point) {
                    if neighbour != start && !previous.contains_key(&neighbour) {
                        previous.insert(neighbour.clone(), point.clone());
                        let new_distance = tentative_distance + distance;
                        queue.push(PointWithTentativeDistance {
                            point: neighbour,
                            tentative_distance: new_distance,
                        });
                    }
                }
            }
        }

        None
    }
}

fn part1(input: &Vec<String>) -> Result<usize, Day12Error> {
    let map: ElevationMap = input.as_slice().try_into().unwrap();

    map.length_of_shortest_path(map.start)
        .ok_or(Day12Error::NoPath)
}

fn part2(input: &Vec<String>) -> Result<usize, Day12Error> {
    let map: ElevationMap = input.as_slice().try_into().unwrap();

    let positions = (0..map.width).flat_map(|x| (0..map.height).map(move |y| (x, y)));
    positions
        .filter(|(x, y)| map.get(*x, *y) == Some(&b'a'))
        .filter_map(|point| map.length_of_shortest_path(point))
        .min()
        .ok_or(Day12Error::NoPath)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_elevation_map_find_start() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        let map: ElevationMap = input.as_slice().try_into().unwrap();

        assert_eq!(map.start, (0, 0));
    }

    #[test]
    fn test_elevation_map_find_end() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        let map: ElevationMap = input.as_slice().try_into().unwrap();

        assert_eq!(map.end, (5, 2));
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part1(&input).unwrap(), 31);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_owned()).collect();

        assert_eq!(part2(&input).unwrap(), 29);
    }
}
