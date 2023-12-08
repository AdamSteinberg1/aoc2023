use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use num_traits::PrimInt;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn ways_to_win<I: PrimInt>(time: I, winning_distance: I) -> usize {
    num_iter::range(I::one(), time)
        .filter(|&speed| speed * (time - speed) > winning_distance)
        .count()
}

fn part1(input: &str) -> Result<usize> {
    let (times, distances) = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).map(str::parse::<u32>))
        .collect_tuple()
        .context("Error: expected two lines in input")?;
    times
        .zip(distances)
        .map(|(time, distance)| Ok(ways_to_win(time?, distance?)))
        .process_results(|iter| iter.product())
}

fn part2(input: &str) -> Result<usize> {
    let [time, distance] = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .map_err(Into::into)
        })
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| anyhow!("Error parsing input"))?;

    Ok(ways_to_win(time, distance))
}
