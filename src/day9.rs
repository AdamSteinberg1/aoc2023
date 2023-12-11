use anyhow::Result;
use itertools::Itertools;
use std::iter::successors;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day9.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn predict_next(nums: Vec<i32>) -> i32 {
    successors(Some(nums), |nums| {
        let row = nums
            .iter()
            .tuple_windows()
            .map(|(prev, next)| next - prev)
            .collect::<Vec<_>>();
        if row.iter().all(|num| *num == 0) {
            return None;
        }
        Some(row)
    })
    .map(|nums| nums.last().copied().expect("nums should not be empty"))
    .sum()
}

fn predict_prev(nums: Vec<i32>) -> i32 {
    successors(Some(nums), |nums| {
        let row = nums
            .iter()
            .tuple_windows()
            .map(|(prev, next)| next - prev)
            .collect::<Vec<_>>();
        if row.iter().all(|num| *num == 0) {
            return None;
        }
        Some(row)
    })
    .map(|nums| nums[0])
    .collect_vec()
    .into_iter()
    .rev()
    .reduce(|a, b| b - a)
    .expect("Vec should not be empty")
}

fn part1(input: &str) -> Result<i32> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()
        })
        .map_ok(predict_next)
        .sum::<Result<_, _>>()
        .map_err(Into::into)
}

fn part2(input: &str) -> Result<i32> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()
        })
        .map_ok(predict_prev)
        .sum::<Result<_, _>>()
        .map_err(Into::into)
}
