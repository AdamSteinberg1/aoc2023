use anyhow::{bail, Context, Result};
use itertools::{chain, Itertools};
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day16.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Tile {
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Empty,
}

impl Tile {
    fn reflect(&self, direction: Direction) -> impl Iterator<Item = Direction> {
        match (self, direction) {
            (Tile::BackwardMirror, Direction::Right) => vec![Direction::Down],
            (Tile::BackwardMirror, Direction::Up) => vec![Direction::Left],
            (Tile::BackwardMirror, Direction::Down) => vec![Direction::Right],
            (Tile::BackwardMirror, Direction::Left) => vec![Direction::Up],

            (Tile::ForwardMirror, Direction::Down) => vec![Direction::Left],
            (Tile::ForwardMirror, Direction::Up) => vec![Direction::Right],
            (Tile::ForwardMirror, Direction::Left) => vec![Direction::Down],
            (Tile::ForwardMirror, Direction::Right) => vec![Direction::Up],

            (Tile::VerticalSplitter, Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }

            (Tile::HorizontalSplitter, Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }

            (_, dir) => vec![dir],
        }
        .into_iter()
    }
}

fn part1(input: &str) -> Result<usize> {
    let contraption = parse_input(input)?;
    let energized_tiles = count_energized_tiles((0, 0), Direction::Right, &contraption);
    Ok(energized_tiles)
}

fn part2(input: &str) -> Result<usize> {
    let contraption = parse_input(input)?;
    get_border(&contraption)
        .map(|(position, direction)| count_energized_tiles(position, direction, &contraption))
        .max()
        .context("Error finding maximum")
}

fn get_border(contraption: &Vec<Vec<Tile>>) -> impl Iterator<Item = ((usize, usize), Direction)> {
    let row_count = contraption.len();
    let col_count = contraption[0].len();

    let left_starts = (0..row_count).map(move |i| ((i, 0), Direction::Right));
    let right_starts = (0..row_count).map(move |i| ((i, col_count - 1), Direction::Left));
    let top_starts = (0..col_count).map(move |i| ((0, i), Direction::Down));
    let bot_starts = (0..col_count).map(move |i| ((row_count - 1, i), Direction::Up));

    chain!(left_starts, right_starts, top_starts, bot_starts)
}

fn count_energized_tiles(
    start_position: (usize, usize),
    start_direction: Direction,
    contraption: &[Vec<Tile>],
) -> usize {
    let next_position = |(row, col): (usize, usize), direction: Direction| {
        let (row, col) = match direction {
            Direction::Up => (row.checked_sub(1)?, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.checked_sub(1)?),
            Direction::Right => (row, col + 1),
        };
        contraption.get(row)?.get(col)?;
        Some((row, col))
    };

    let mut visited = HashSet::new();
    let mut stack = vec![(start_position, start_direction)];

    //depth-first search
    while let Some(pair) = stack.pop() {
        if visited.contains(&pair) {
            continue;
        }
        visited.insert(pair);

        let (position, direction) = pair;
        let (row, col) = position;
        let neighbors = contraption[row][col]
            .reflect(direction)
            .filter_map(|dir| next_position(position, dir).map(|pos| (pos, dir)));
        stack.extend(neighbors);
    }

    visited
        .into_iter()
        .unique_by(|&(position, _)| position)
        .count()
}

fn parse_input(input: &str) -> Result<Vec<Vec<Tile>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(match c {
                        '/' => Tile::ForwardMirror,
                        '\\' => Tile::BackwardMirror,
                        '|' => Tile::VerticalSplitter,
                        '-' => Tile::HorizontalSplitter,
                        '.' => Tile::Empty,
                        _ => bail!("Unexpected character: {}", c),
                    })
                })
                .collect()
        })
        .collect()
}
