use std::{io, num::ParseIntError, ops::RangeInclusive, str::FromStr};

use aoc::read_lines;

#[derive(Debug)]
enum Day15Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidPoint,
    InvalidSensorReport,
    EmptyInput,
    BeaconNotFound,
}

impl From<io::Error> for Day15Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day15Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day15.txt";

fn main() -> Result<(), Day15Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input, 2000000)?);
    println!("Part 2: {:?}", part2(&input, 0, 4000000)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl FromStr for Point {
    type Err = Day15Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(&['=', ',']).collect();
        match v.as_slice() {
            ["x", x, " y", y] => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            }),
            _ => Err(Self::Err::InvalidPoint),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SensorReport {
    sensor_position: Point,
    beacon_position: Point,
    exclusion_radius: i64,
}

impl SensorReport {
    fn new(sensor_position: Point, beacon_position: Point) -> Self {
        let exclusion_radius = sensor_position.manhattan_distance(&beacon_position);

        Self {
            sensor_position,
            beacon_position,
            exclusion_radius,
        }
    }

    fn range_at(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let Point {
            x: sensor_x,
            y: sensor_y,
        } = self.sensor_position;

        let remaining_distance = self.exclusion_radius - (y - sensor_y).abs();
        if remaining_distance >= 0 {
            Some(sensor_x - remaining_distance..=sensor_x + remaining_distance)
        } else {
            None
        }
    }
}

impl FromStr for SensorReport {
    type Err = Day15Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(':').collect();
        match v.as_slice() {
            [sensor, beacon] => {
                let sensor_position = sensor
                    .strip_prefix("Sensor at ")
                    .ok_or(Self::Err::InvalidSensorReport)?;

                let beacon_position = beacon
                    .strip_prefix(" closest beacon is at ")
                    .ok_or(Self::Err::InvalidSensorReport)?;

                Ok(Self::new(
                    sensor_position.parse()?,
                    beacon_position.parse()?,
                ))
            }
            _ => Err(Self::Err::InvalidSensorReport),
        }
    }
}

fn parse_sensor_reports(input: &Vec<String>) -> Result<Vec<SensorReport>, Day15Error> {
    input.iter().map(|line| line.parse()).collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Boundaries {
    left: i64,
    top: i64,
    right: i64,
    bottom: i64,
}

fn find_boundaries(sensor_reports: &Vec<SensorReport>) -> Option<Boundaries> {
    let mut left: Option<i64> = None;
    let mut top: Option<i64> = None;
    let mut right: Option<i64> = None;
    let mut bottom: Option<i64> = None;

    for report in sensor_reports {
        let Point { x, y } = report.sensor_position;
        let radius = report.exclusion_radius;

        left = Some(left.map(|left| left.min(x - radius)).unwrap_or(x - radius));
        top = Some(top.map(|top| top.min(y - radius)).unwrap_or(y - radius));
        right = Some(
            right
                .map(|right| right.max(x + radius))
                .unwrap_or(x + radius),
        );
        bottom = Some(
            bottom
                .map(|bottom| bottom.max(y + radius))
                .unwrap_or(y + radius),
        );
    }

    Some(Boundaries {
        left: left?,
        top: top?,
        right: right?,
        bottom: bottom?,
    })
}

enum TileContents {
    Beacon,
    NotBeacon,
    Unknown,
}

fn get_tile_contents(sensor_reports: &Vec<SensorReport>, position: Point) -> TileContents {
    use TileContents::*;

    for report in sensor_reports {
        if position == report.beacon_position {
            return Beacon;
        }

        if position.manhattan_distance(&report.sensor_position) <= report.exclusion_radius {
            return NotBeacon;
        }
    }

    Unknown
}

fn part1(input: &Vec<String>, y: i64) -> Result<usize, Day15Error> {
    let sensor_reports = parse_sensor_reports(input)?;
    let boundaries = find_boundaries(&sensor_reports).ok_or(Day15Error::EmptyInput)?;

    let row = (boundaries.left..boundaries.right)
        .map(|x| Point { x, y })
        .map(|position| get_tile_contents(&sensor_reports, position));

    Ok(row
        .filter(|contents| matches!(contents, TileContents::NotBeacon))
        .count())
}

fn is_contained(ranges: &Vec<RangeInclusive<i64>>, candidate: &i64) -> bool {
    for range in ranges {
        if range.contains(candidate) {
            return true;
        }
    }

    false
}

fn find_gap(ranges: &Vec<RangeInclusive<i64>>, search_min: i64, search_max: i64) -> Option<i64> {
    for range in ranges {
        let candidate_x = range.start() - 1;
        if candidate_x >= search_min && !is_contained(ranges, &candidate_x) {
            return Some(candidate_x);
        }

        let candidate_x = range.end() + 1;
        if candidate_x <= search_max && !is_contained(ranges, &candidate_x) {
            return Some(candidate_x);
        }
    }

    None
}

fn part2(input: &Vec<String>, search_min: i64, search_max: i64) -> Result<i64, Day15Error> {
    let sensor_reports = parse_sensor_reports(input)?;

    for y in search_min..=search_max {
        let ranges = sensor_reports
            .iter()
            .filter_map(|report| report.range_at(y))
            .collect();

        if let Some(x) = find_gap(&ranges, search_min, search_max) {
            return Ok(x * 4000000 + y);
        }
    }

    Err(Day15Error::BeaconNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    const EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_parse_sensor_report() {
        let value: SensorReport = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
            .parse()
            .unwrap();
        let expected = SensorReport {
            sensor_position: Point { x: 2, y: 18 },
            beacon_position: Point { x: -2, y: 15 },
            exclusion_radius: 7,
        };

        assert_eq!(value, expected);
    }

    #[test]
    fn test_boundaries() {
        let input = to_lines(EXAMPLE);
        let sensor_reports = parse_sensor_reports(&input).unwrap();

        let boundaries = find_boundaries(&sensor_reports).unwrap();
        let expected = Boundaries {
            left: -8,
            top: -10,
            right: 28,
            bottom: 26,
        };

        assert_eq!(boundaries, expected);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input, 10).unwrap(), 26);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input, 0, 20).unwrap(), 56000011);
    }
}
