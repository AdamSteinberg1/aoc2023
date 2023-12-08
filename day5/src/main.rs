use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn parse_transforms(input: &str) -> Result<Vec<impl Fn(u64) -> u64>> {
    input
        .split("\n\n")
        .skip(1)
        .map(|map| {
            let values = map
                .lines()
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(str::parse)
                        .process_results(|iter| iter.collect_tuple())?
                        .context(format!("Error parsing almanac line: {}", line))
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(move |num| {
                for (dest, source, length) in &values {
                    let source_range = *source..(*source + *length);
                    if source_range.contains(&num) {
                        return num + dest - source;
                    }
                }
                num
            })
        })
        .collect()
}

fn parse_seeds(input: &str) -> Result<impl Iterator<Item = &str>> {
    Ok(input
        .lines()
        .next()
        .context("Empty input")?
        .split_whitespace()
        .skip(1))
}

fn part1(input: &str) -> Result<u64> {
    let transforms = parse_transforms(input)?;

    parse_seeds(input)?
        .map(str::parse)
        .map_ok(|seed| {
            transforms
                .iter()
                .fold(seed, |acc, transform| transform(acc))
        })
        .process_results(|iter| iter.min())?
        .context("Empty location iterator")
}

fn part2(input: &str) -> Result<u64> {
    let transforms = parse_transforms(input)?;
    let seed_ranges = parse_seeds(input)?
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let (start, length) = chunk
                .collect_tuple()
                .context("Error chunking seeds into pairs")?;
            let start = start.parse::<u64>()?;
            let length = length.parse::<u64>()?;
            Ok(start..start + length)
        })
        .collect::<Result<Vec<_>>>()?;

    seed_ranges
        .into_par_iter()
        .flatten()
        .map(|seed| {
            transforms
                .iter()
                .fold(seed, |acc, transform| transform(acc))
        })
        .min()
        .context("Empty finding minimum location")
}
