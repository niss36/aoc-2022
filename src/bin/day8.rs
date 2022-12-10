use std::{io, num::ParseIntError, ops::Range};

use aoc::read_lines;

#[derive(Debug)]
enum Day8Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InconsistentRowWidth,
    IndexOutOfBounds,
    NoSolution,
}

impl From<io::Error> for Day8Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Day8Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day8.txt";

fn main() -> Result<(), Day8Error> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

struct Grid {
    store: Vec<usize>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new<I, J, E>(iter: I) -> Result<Self, Day8Error>
    where
        I: Iterator<Item = J>,
        J: Iterator<Item = Result<usize, E>>,
        Day8Error: From<E>,
    {
        let mut width: Option<usize> = None;
        let mut height: usize = 0;
        let mut store: Vec<usize> = vec![];

        for row in iter {
            height += 1;
            let mut row_width = 0;
            for item in row {
                row_width += 1;
                store.push(item?);
            }

            match width {
                None => {
                    width = Some(row_width);
                }
                Some(width) if width != row_width => return Err(Day8Error::InconsistentRowWidth),
                _ => {}
            }
        }

        let width = width.unwrap_or(0);

        debug_assert!(store.len() == width * height);

        Ok(Grid {
            store,
            width,
            height,
        })
    }

    fn get(&self, row_index: usize, col_index: usize) -> Option<&usize> {
        let index = row_index * self.width + col_index;
        debug_assert!(index < self.width * self.height);

        self.store.get(index)
    }
}

fn parse_forest_map(input: &Vec<String>) -> Result<Grid, Day8Error> {
    Grid::new(
        input
            .iter()
            .map(|line| line.chars().map(|c| c.to_string().parse())),
    )
}

fn is_visible(grid: &Grid, row_index: usize, col_index: usize) -> bool {
    let tree_height = grid.get(row_index, col_index);

    if let Some(tree_height) = tree_height {
        let is_visible_along = |range: Range<usize>, is_row: bool| {
            range
                .filter_map(|index| {
                    if is_row {
                        grid.get(row_index, index)
                    } else {
                        grid.get(index, col_index)
                    }
                })
                .max()
                .map(|highest_tree| tree_height > highest_tree)
                .unwrap_or(true)
        };

        is_visible_along(0..row_index, false)
            || is_visible_along(row_index + 1..grid.height, false)
            || is_visible_along(0..col_index, true)
            || is_visible_along(col_index + 1..grid.width, true)
    } else {
        false
    }
}

fn part1(input: &Vec<String>) -> Result<usize, Day8Error> {
    let grid = parse_forest_map(input)?;

    Ok((0..grid.height)
        .flat_map(|row_index| (0..grid.width).map(move |col_index| (row_index, col_index)))
        .filter(|(row_index, col_index)| is_visible(&grid, *row_index, *col_index))
        .count())
}

fn viewing_distance_along<I>(
    grid: &Grid,
    row_index: usize,
    col_index: usize,
    tree_height: &usize,
    iter: I,
    is_row: bool,
) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut viewing_distance: usize = 0;

    for height in iter.filter_map(|index| {
        if is_row {
            grid.get(row_index, index)
        } else {
            grid.get(index, col_index)
        }
    }) {
        viewing_distance += 1;
        if height >= tree_height {
            break;
        }
    }

    viewing_distance
}

fn scenic_score(grid: &Grid, row_index: usize, col_index: usize) -> Result<usize, Day8Error> {
    match grid.get(row_index, col_index) {
        Some(tree_height) => Ok(viewing_distance_along(
            grid,
            row_index,
            col_index,
            tree_height,
            (0..row_index).rev(),
            false,
        ) * viewing_distance_along(
            grid,
            row_index,
            col_index,
            tree_height,
            row_index + 1..grid.height,
            false,
        ) * viewing_distance_along(
            grid,
            row_index,
            col_index,
            tree_height,
            (0..col_index).rev(),
            true,
        ) * viewing_distance_along(
            grid,
            row_index,
            col_index,
            tree_height,
            col_index + 1..grid.width,
            true,
        )),
        None => Err(Day8Error::IndexOutOfBounds),
    }
}

fn part2(input: &Vec<String>) -> Result<usize, Day8Error> {
    let grid = parse_forest_map(input)?;

    let scenic_scores = (0..grid.height)
        .flat_map(|row_index| (0..grid.width).map(move |col_index| (row_index, col_index)))
        .map(|(row_index, col_index)| scenic_score(&grid, row_index, col_index))
        .collect::<Result<Vec<usize>, Day8Error>>()?;

    scenic_scores.into_iter().max().ok_or(Day8Error::NoSolution)
}
