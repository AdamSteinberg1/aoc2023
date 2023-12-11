use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::{collections::HashSet, iter::successors};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day10.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum Pipe {
    Southeast,
    Northeast,
    Southwest,
    Northwest,
    Eastwest,
    Northsouth,
    Start,
    None,
}

impl Pipe {
    fn north_facing(&self) -> bool {
        matches!(
            self,
            Self::Northeast | Self::Northwest | Self::Northsouth | Self::Start
        )
    }

    fn connected_to_south(&self) -> bool {
        matches!(self, Pipe::Southeast | Pipe::Southwest | Pipe::Northsouth)
    }

    fn connected_to_north(&self) -> bool {
        matches!(self, Pipe::Northwest | Pipe::Northeast | Pipe::Northsouth)
    }

    fn connected_to_west(&self) -> bool {
        matches!(self, Pipe::Northwest | Pipe::Southwest | Pipe::Eastwest)
    }

    fn connected_to_east(&self) -> bool {
        matches!(self, Pipe::Northeast | Pipe::Southeast | Pipe::Eastwest)
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            'F' => Pipe::Southeast,
            'L' => Pipe::Northeast,
            '-' => Pipe::Eastwest,
            '7' => Pipe::Southwest,
            'J' => Pipe::Northwest,
            '|' => Pipe::Northsouth,
            'S' => Pipe::Start,
            _ => Pipe::None,
        }
    }
}

fn main_path(grid: &Vec<Vec<Pipe>>) -> Result<impl Iterator<Item = (usize, usize)> + '_> {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let start = (0..row_count)
        .cartesian_product(0..col_count)
        .find(|&(i, j)| grid[i][j] == Pipe::Start)
        .context("No start in input")?;

    let start_direction = {
        let (i, j) = start;

        //assume start is not on an edge
        let northern = &grid[i - 1][j];
        let southern = &grid[i + 1][j];
        let eastern = &grid[i][j + 1];
        let western = &grid[i][j - 1];

        if northern.connected_to_south() {
            Direction::North
        } else if southern.connected_to_north() {
            Direction::South
        } else if eastern.connected_to_west() {
            Direction::East
        } else if western.connected_to_east() {
            Direction::West
        } else {
            bail!("No pipes connected to start");
        }
    };

    let path = successors(Some((start, start_direction)), |(position, direction)| {
        let &(row, col) = position;
        let next_position = match direction {
            Direction::North => (row - 1, col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
        };

        let (next_row, next_col) = next_position;
        let next_pipe = &grid[next_row][next_col];
        let next_direction = match (direction, next_pipe) {
            (_, Pipe::Start) => return None,

            (Direction::North, Pipe::Southeast) => Direction::East,
            (Direction::North, Pipe::Southwest) => Direction::West,
            (Direction::North, Pipe::Northsouth) => Direction::North,

            (Direction::East, Pipe::Northwest) => Direction::North,
            (Direction::East, Pipe::Southwest) => Direction::South,
            (Direction::East, Pipe::Eastwest) => Direction::East,

            (Direction::South, Pipe::Northwest) => Direction::West,
            (Direction::South, Pipe::Northeast) => Direction::East,
            (Direction::South, Pipe::Northsouth) => Direction::South,

            (Direction::West, Pipe::Northeast) => Direction::North,
            (Direction::West, Pipe::Southeast) => Direction::South,
            (Direction::West, Pipe::Eastwest) => Direction::West,

            _ => panic!("Dead end"),
        };

        Some((next_position, next_direction))
    })
    .map(|(position, _)| position);
    Ok(path)
}

fn parse_grid(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect()
}

fn inside_area(grid: Vec<Vec<Pipe>>) -> Result<usize> {
    let main_path = main_path(&grid)?.collect::<HashSet<_>>();
    let area = grid
        .into_iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .scan(false, |inside, (col_index, pipe)| {
                    let is_main_path = main_path.contains(&(row_index, col_index));
                    if (pipe.north_facing() || *pipe == Pipe::Start) && is_main_path {
                        *inside ^= true;
                    }
                    Some(*inside && !is_main_path)
                })
                .filter(|is_inside| *is_inside)
                .count()
        })
        .sum();
    Ok(area)
}

fn part1(input: &str) -> Result<usize> {
    let grid = parse_grid(input);
    let main_path = main_path(&grid)?;
    let farthest = main_path.count() / 2;
    Ok(farthest)
}

fn part2(input: &str) -> Result<usize> {
    let grid = parse_grid(input);
    inside_area(grid)
}
