use anyhow::{bail, Context, Result};
use itertools::Itertools;
use memoize::memoize;
use std::iter::repeat;
use tap::Pipe;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day12.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

#[derive(PartialEq, Clone, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_line(line: &str) -> Result<(Vec<Spring>, Vec<usize>)> {
    let (springs, counts) = line
        .split_once(' ')
        .context(format!("Error parsing line: {}", line))?;
    let springs = springs
        .chars()
        .map(|c| {
            match c {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => bail!("Unexpected character: '{}'", c),
            }
            .pipe(Ok)
        })
        .collect::<Result<_>>()?;

    let counts = counts
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    Ok((springs, counts))
}

#[memoize]
fn count_arrangements(springs: Vec<Spring>, counts: Vec<usize>) -> usize {
    if counts.is_empty() {
        if springs.contains(&Spring::Damaged) {
            return 0;
        } else {
            return 1;
        }
    }

    if springs.is_empty() {
        return 0;
    }

    match springs[0] {
        Spring::Damaged => count_assuming_damaged(&springs, &counts),
        Spring::Operational => count_arrangements(springs[1..].to_vec(), counts),
        Spring::Unknown => {
            count_assuming_damaged(&springs, &counts)
                + count_arrangements(springs[1..].to_vec(), counts)
        }
    }
}

fn count_assuming_damaged(springs: &[Spring], counts: &[usize]) -> usize {
    let first_count = counts[0];

    let Some(first_group) = springs.get(..first_count) else {
        return 0;
    };

    if first_group.contains(&Spring::Operational) {
        return 0;
    }

    if springs.len() == first_count {
        if counts.len() == 1 {
            return 1;
        } else {
            return 0;
        };
    }

    match &springs[first_count] {
        &Spring::Operational | &Spring::Unknown => {
            count_arrangements(springs[first_count + 1..].to_vec(), counts[1..].to_vec())
        }
        &Spring::Damaged => 0,
    }
}

fn unfold(springs: Vec<Spring>, counts: Vec<usize>) -> (Vec<Spring>, Vec<usize>) {
    let springs = repeat(springs)
        .take(5)
        .intersperse(vec![Spring::Unknown])
        .flatten()
        .collect::<Vec<_>>();
    let counts = repeat(counts).take(5).flatten().collect::<Vec<_>>();
    (springs, counts)
}

fn part1(input: &str) -> Result<usize> {
    input
        .lines()
        .map(parse_line)
        .map_ok(|(springs, counts)| count_arrangements(springs, counts))
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    input
        .lines()
        .map(parse_line)
        .map_ok(|(springs, counts)| unfold(springs, counts))
        .map_ok(|(springs, counts)| count_arrangements(springs, counts))
        .sum()
}
