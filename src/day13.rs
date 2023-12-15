use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day13.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn pattern_to_string(pattern: &[Vec<char>]) -> String {
    pattern
        .iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect()
}

fn summarize(pattern: &Vec<Vec<char>>) -> Result<usize> {
    if let Some(columns) = check_vertical_symmetry(pattern) {
        return Ok(columns);
    }
    if let Some(rows) = check_horizontal_symmetry(pattern) {
        return Ok(100 * rows);
    }

    bail!(
        "No symmetry in this pattern:\n{}",
        pattern_to_string(pattern)
    );
}

fn check_vertical_symmetry(pattern: &[Vec<char>]) -> Option<usize> {
    let col_count = pattern[0].len();

    for i in 1..col_count {
        let is_mirrored = pattern.iter().all(|row| {
            let left_half = row[..i].iter().rev();
            let right_half = row[i..].iter();

            left_half.zip(right_half).all(|(left, right)| left == right)
        });

        if is_mirrored {
            return Some(i);
        }
    }
    None
}

fn check_horizontal_symmetry(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let row_count = pattern.len();

    for i in 1..row_count {
        let top_half = pattern[..i].iter().rev();
        let bottom_half = pattern[i..].iter();
        let is_mirrored = top_half.zip(bottom_half).all(|(top, bottom)| top == bottom);

        if is_mirrored {
            return Some(i);
        }
    }
    None
}

fn parse_pattern(pattern: &str) -> Vec<Vec<char>> {
    pattern.lines().map(|line| line.chars().collect()).collect()
}

fn summarize_with_smudge(pattern: &[Vec<char>]) -> Result<usize> {
    if let Some(vertical_line) = check_vertical_symmetry_with_smudge(pattern) {
        return Ok(vertical_line);
    }

    if let Some(horizontal_line) = check_horizontal_symmetry_with_smudge(pattern) {
        return Ok(horizontal_line * 100);
    }

    bail!(
        "No symmetry in this pattern:\n{}",
        pattern_to_string(pattern)
    );
}

fn check_vertical_symmetry_with_smudge(pattern: &[Vec<char>]) -> Option<usize> {
    let col_count = pattern[0].len();

    for i in 1..col_count {
        let length = i.min(col_count - i);
        let left_half = pattern
            .iter()
            .flat_map(|row| row[..i].iter().rev().take(length));
        let right_half = pattern.iter().flat_map(|row| row[i..].iter().take(length));

        let differences = left_half
            .zip(right_half)
            .filter(|(left, right)| left != right)
            .count();

        if differences == 1 {
            return Some(i);
        }
    }
    None
}

fn check_horizontal_symmetry_with_smudge(pattern: &[Vec<char>]) -> Option<usize> {
    let row_count = pattern.len();

    for i in 1..row_count {
        let top_half = pattern[..i].iter().rev();
        let bottom_half = pattern[i..].iter();
        let differences = top_half
            .zip(bottom_half)
            .map(|(top, bottom)| {
                top.iter()
                    .zip(bottom.iter())
                    .filter(|(top, bottom)| top != bottom)
                    .count()
            })
            .sum::<usize>();

        if differences == 1 {
            return Some(i);
        }
    }
    None
}

fn part1(input: &str) -> Result<usize> {
    input
        .split("\n\n")
        .map(parse_pattern)
        .map(|pattern| summarize(&pattern))
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    input
        .split("\n\n")
        .map(parse_pattern)
        .map(|pattern| summarize_with_smudge(&pattern))
        .sum()
}
