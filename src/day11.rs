use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day11.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

struct Galaxy;

fn parse_image(input: &str) -> Result<Vec<Vec<Option<Galaxy>>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Ok(Some(Galaxy)),
                    '.' => Ok(None),
                    _ => Err(anyhow!("Unexpected character: '{}'", c)),
                })
                .collect()
        })
        .collect()
}

fn manhattan_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    y1.abs_diff(y2) + x1.abs_diff(x2)
}

fn expand(init: usize, expansion: usize, expanded_lines: impl Iterator<Item = usize>) -> usize {
    expanded_lines.fold(init, |acc, expanded_line| {
        if acc > expanded_line {
            acc + expansion
        } else {
            acc
        }
    })
}

fn locate_galaxies(image: &Vec<Vec<Option<Galaxy>>>, expansion: usize) -> Vec<(usize, usize)> {
    let row_count = image.len();
    let col_count = image[0].len();

    let empty_rows = (0..row_count)
        .rev()
        .filter(|&i| image[i].iter().all(|point| point.is_none()));

    let empty_cols = (0..col_count).rev().filter(|&i| {
        image
            .iter()
            .map(move |row| &row[i])
            .all(|point| point.is_none())
    });

    (0..row_count)
        .cartesian_product(0..col_count)
        .filter(|&(row, col)| image[row][col].is_some())
        .map(|(row, col)| {
            let row = expand(row, expansion, empty_rows.clone());
            let col = expand(col, expansion, empty_cols.clone());
            (row, col)
        })
        .collect()
}

fn sum_distances(galaxies: &[(usize, usize)]) -> usize {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(&p1, &p2)| manhattan_distance(p1, p2))
        .sum()
}

fn part1(input: &str) -> Result<usize> {
    let image = parse_image(input)?;
    let galaxies = locate_galaxies(&image, 1);
    let sum = sum_distances(&galaxies);
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let image = parse_image(input)?;
    let galaxies = locate_galaxies(&image, 999999);
    let sum = sum_distances(&galaxies);
    Ok(sum)
}
