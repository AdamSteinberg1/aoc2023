use anyhow::{bail, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day2.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

#[derive(Default)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Sample {
    fn product(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    samples: Vec<Sample>,
}

fn parse_game(line: &str) -> Result<Game> {
    let (id, remaining) = line.split_once(':').context("No \":\" in line.")?;
    let id = id
        .split_once(' ')
        .context("Error parsing id")?
        .1
        .parse::<u32>()?;

    let samples = remaining
        .split(';')
        .map(|sample| {
            sample
                .split(',')
                .map(|cube| {
                    let (count, color) = cube
                        .trim()
                        .split_once(' ')
                        .context("Error parsing cube count")?;
                    let count = count.parse()?;
                    Ok((count, color))
                })
                .try_fold(Sample::default(), |acc, cube: Result<_>| {
                    let (count, color) = cube?;
                    match color {
                        "red" => Ok(Sample { red: count, ..acc }),
                        "green" => Ok(Sample {
                            green: count,
                            ..acc
                        }),
                        "blue" => Ok(Sample { blue: count, ..acc }),
                        _ => bail!("Unexpected color: {}", color),
                    }
                })
        })
        .collect::<Result<_>>()?;

    Ok(Game { samples, id })
}

fn part1(input: &str) -> Result<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    input
        .lines()
        .map(parse_game)
        .filter_map_ok(|Game { id, samples }| {
            for Sample { red, green, blue } in samples {
                if red > max_red || green > max_green || blue > max_blue {
                    return None;
                }
            }
            Some(id)
        })
        .sum()
}

fn part2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(parse_game)
        .map_ok(|Game { samples, .. }| {
            samples
                .into_iter()
                .fold(
                    Sample::default(),
                    |Sample {
                         red: max_red,
                         green: max_green,
                         blue: max_blue,
                     },
                     Sample { red, green, blue }| {
                        Sample {
                            red: max_red.max(red),
                            green: max_green.max(green),
                            blue: max_blue.max(blue),
                        }
                    },
                )
                .product()
        })
        .sum()
}
