use anyhow::{bail, Context, Result};
use itertools::Itertools;
use num_integer::Integer;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day8.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_steps(input: &str) -> Result<impl Iterator<Item = Direction> + Clone> {
    input
        .lines()
        .next()
        .context("Empty input")?
        .chars()
        .map(|c| {
            Ok(match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => bail!("Unexpected character: {}", c),
            })
        })
        .collect::<Result<Vec<_>>>()
        .map(|steps| steps.into_iter().cycle())
}

fn parse_map(input: &str) -> Result<HashMap<&str, (&str, &str)>> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let Some((source, left, right)) = line
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect_tuple()
            else {
                bail!("Error parsing line: {}", line);
            };
            Ok((source, (left, right)))
        })
        .collect()
}

fn count_steps(
    steps: impl Iterator<Item = Direction>,
    start: &str,
    map: &HashMap<&str, (&str, &str)>,
    is_end: impl Fn(&str) -> bool,
) -> usize {
    steps
        .scan(start, |location, direction| {
            if is_end(location) {
                return None;
            }
            let (left, right) = map[location];
            match direction {
                Direction::Left => *location = left,
                Direction::Right => *location = right,
            }
            Some(())
        })
        .count()
}

fn part1(input: &str) -> Result<usize> {
    let map = parse_map(input)?;
    let steps = parse_steps(input)?;
    let count = count_steps(steps, "AAA", &map, |location| location == "ZZZ");
    Ok(count)
}

fn part2(input: &str) -> Result<usize> {
    let map = parse_map(input)?;
    let steps = parse_steps(input)?;
    let start_locations = input.lines().skip(2).filter_map(|line| {
        line.split_whitespace()
            .next()
            .filter(|location| location.ends_with('A'))
    });

    let count = start_locations
        .map(|start| {
            count_steps(steps.clone(), start, &map, |location| {
                location.ends_with('Z')
            })
        })
        .reduce(|a, b| a.lcm(&b))
        .context("No start locations")?;
    Ok(count)
}
