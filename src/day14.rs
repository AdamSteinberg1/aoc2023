use anyhow::{bail, Result};
use itertools::Itertools;
use std::{collections::HashMap, iter::successors};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day14.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut rocks = parse_input(input)?;
    tilt(&mut rocks, Direction::North);
    let load = calculate_load(&rocks);
    Ok(load)
}

fn part2(input: &str) -> Result<usize> {
    let mut rocks = parse_input(input)?;

    cycle_rocks(&mut rocks);

    let load = calculate_load(&rocks);

    Ok(load)
}

type RockPlatform = Vec<Vec<Option<Rock>>>;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Round,
    Cube,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_input(input: &str) -> Result<RockPlatform> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(match c {
                        'O' => Some(Rock::Round),
                        '#' => Some(Rock::Cube),
                        '.' => None,
                        _ => bail!("Unexpected char: '{}'", c),
                    })
                })
                .collect()
        })
        .collect()
}

fn tilt(rocks: &mut RockPlatform, direction: Direction) {
    let direction = match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    };

    let row_count = rocks.len();
    let column_count = rocks[0].len();

    for (i, j) in (0..row_count).cartesian_product(0..column_count) {
        if rocks[i][j] != Some(Rock::Round) {
            continue;
        }

        rocks[i][j] = None;

        let cube_loc = successors(Some((i, j)), |(i, j)| {
            let i = i.checked_add_signed(direction.0)?;
            let j = j.checked_add_signed(direction.1)?;
            rocks.get(i)?.get(j).map(|_| (i, j))
        })
        .find_or_last(|&(i, j)| rocks[i][j] == Some(Rock::Cube));

        let (i, j) = successors(cube_loc, |(i, j)| {
            let i = i.checked_add_signed(-direction.0)?;
            let j = j.checked_add_signed(-direction.1)?;
            rocks.get(i)?.get(j).map(|_| (i, j))
        })
        .find_or_last(|&(i, j)| rocks[i][j].is_none())
        .expect("Iterator should never be empty");
        rocks[i][j] = Some(Rock::Round);
    }
}

fn calculate_load(rocks: &RockPlatform) -> usize {
    let column_count = rocks[0].len();
    let columns = (0..column_count).map(|i| rocks.iter().map(move |row| &row[i]));

    columns
        .map(|column| {
            column
                .rev()
                .enumerate()
                .fold(0, |acc, (i, rock)| match rock {
                    Some(Rock::Round) => acc + i + 1,
                    _ => acc,
                })
        })
        .sum()
}

//execute the spin cycle 1000000000 times
fn cycle_rocks(rocks: &mut Vec<Vec<Option<Rock>>>) {
    let (start, end) = find_cycle(rocks);
    let cycle_length = end - start;
    let leftover = (1_000_000_000 - start) % cycle_length;

    for _ in 0..leftover {
        spin_cycle(rocks);
    }
}

fn find_cycle(rocks: &mut Vec<Vec<Option<Rock>>>) -> (usize, usize) {
    let mut i = 0;
    let mut map = HashMap::new();

    loop {
        if let Some(start) = map.get(rocks) {
            return (*start, i);
        }

        map.insert(rocks.clone(), i);
        i += 1;
        spin_cycle(rocks);
    }
}

fn spin_cycle(rocks: &mut RockPlatform) {
    tilt(rocks, Direction::North);
    tilt(rocks, Direction::West);
    tilt(rocks, Direction::South);
    tilt(rocks, Direction::East);
}
