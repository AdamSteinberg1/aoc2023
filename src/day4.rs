use std::collections::HashSet;

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day4.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let (winning_nums, my_nums) =
                line.split_once('|').context("Error splitting on \'|\'")?;
            let winning_nums = winning_nums
                .split_once(':')
                .context("Error splitting on \':\'")?
                .1
                .split_whitespace()
                .map(|num| num.parse())
                .collect::<Result<HashSet<u32>, _>>()?;

            my_nums
                .split_whitespace()
                .map(|num| num.parse::<u32>())
                .filter_ok(|num| winning_nums.contains(num))
                .process_results(|iter| iter.count())
                .map(|count| match count {
                    0 => 0,
                    _ => 2_u32.pow(count as u32 - 1),
                })
                .map_err(Into::into)
        })
        .sum()
}

fn part2(input: &str) -> Result<u32> {
    let card_count = input.lines().count();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| -> Result<_> {
            let (winning_nums, my_nums) =
                line.split_once('|').context("Error splitting on \'|\'")?;
            let (_, winning_nums) = winning_nums
                .split_once(':')
                .context("Error splitting on \':\'")?;
            let winning_nums = winning_nums
                .split_whitespace()
                .map(|num| num.parse())
                .collect::<Result<HashSet<u32>, _>>()?;

            my_nums
                .split_whitespace()
                .map(|num| num.parse::<u32>())
                .filter_ok(|num| winning_nums.contains(num))
                .process_results(|iter| iter.count())
                .map(|matches| (i, matches))
                .map_err(Into::into)
        })
        .fold_ok(vec![1; card_count], |mut counts, (card_index, matches)| {
            let count = counts[card_index];
            for i in (card_index + 1)..=(card_index + matches) {
                if let Some(elem) = counts.get_mut(i) {
                    *elem += count;
                }
            }
            counts
        })
        .map(|map| map.into_iter().sum())
}
