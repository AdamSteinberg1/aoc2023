use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day1.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit);
            let last = line.chars().rfind(char::is_ascii_digit);
            let (first, last) = first
                .zip(last)
                .context(format!("No digit in line: {}", line))?;

            let num = format!("{}{}", first, last);
            let num = num.parse::<u32>()?;
            Ok(num)
        })
        .sum()
}

fn part2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            const NUMS: [(&str, &str); 9] = [
                ("1", "one"),
                ("2", "two"),
                ("3", "three"),
                ("4", "four"),
                ("5", "five"),
                ("6", "six"),
                ("7", "seven"),
                ("8", "eight"),
                ("9", "nine"),
            ];

            let first = NUMS
                .into_iter()
                .flat_map(|(num, word)| {
                    line.find(num)
                        .into_iter()
                        .chain(line.find(word))
                        .map(move |index| (index, num))
                })
                .min_by_key(|(index, _)| *index)
                .map(|(_, num)| num)
                .context("Error finding minimum index")?;

            let last = NUMS
                .into_iter()
                .flat_map(|(num, word)| {
                    line.rfind(num)
                        .into_iter()
                        .chain(line.rfind(word))
                        .map(move |index| (index, num))
                })
                .max_by_key(|(index, _)| *index)
                .map(|(_, num)| num)
                .context("Error finding maximum index")?;

            let num = format!("{}{}", first, last);
            let num = num.parse::<u32>()?;
            Ok(num)
        })
        .sum()
}
