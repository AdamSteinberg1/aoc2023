use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let col_count = grid[0].len();
    let row_count = grid.len();
    (0..row_count)
        .cartesian_product(0..col_count)
        .filter_map(|(row_index, col_index)| {
            let num_start = grid[row_index][col_index];
            if !num_start.is_ascii_digit() {
                return None;
            }

            let predecessor = col_index
                .checked_sub(1)
                .and_then(|predecessor_index| grid[row_index].get(predecessor_index));
            if predecessor.is_some_and(|c| c.is_ascii_digit()) {
                return None;
            }

            let num = {
                let row = &grid[row_index];
                (col_index..)
                    .map_while(|offset| row.get(offset).filter(|c| c.is_ascii_digit()))
                    .collect::<String>()
            };

            let num_length = num.chars().count();
            (-1..=1)
                .cartesian_product(-1..=num_length as isize)
                .filter_map(|(i, j)| {
                    grid.get(row_index.checked_add_signed(i)?)?
                        .get(col_index.checked_add_signed(j)?)
                })
                .any(|c| !c.is_ascii_digit() && *c != '.')
                .then(|| num.parse::<u32>().map_err(Into::into))
        })
        .sum()
}

fn part2(input: &str) -> Result<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let col_count = grid[0].len();
    let row_count = grid.len();
    (0..row_count)
        .cartesian_product(0..col_count)
        .filter_map(|(row_index, col_index)| {
            let gear = grid[row_index][col_index];
            if gear != '*' {
                return None;
            }

            let neighboring_nums = (row_index.saturating_sub(1)..=row_index + 1)
                .cartesian_product(0..col_count)
                .filter_map(|(i, j)| {
                    let num_start = grid.get(i)?.get(j)?;

                    if !num_start.is_ascii_digit() {
                        return None;
                    }

                    let predecessor = grid.get(i).and_then(|row| row.get(j.checked_sub(1)?));
                    if predecessor.is_some_and(|c| c.is_ascii_digit()) {
                        return None;
                    }

                    let num = {
                        let row = &grid.get(i)?;
                        (j..)
                            .map_while(|offset| row.get(offset).filter(|c| c.is_ascii_digit()))
                            .collect::<String>()
                    };

                    if j > col_index + 1 {
                        return None;
                    }

                    let num_length = num.chars().count();
                    if j + num_length < col_index {
                        return None;
                    }

                    Some(num)
                });

            let (first, second) = neighboring_nums.collect_tuple()?;
            Some(
                first
                    .parse::<u32>()
                    .and_then(|first| Ok(first * second.parse::<u32>()?))
                    .map_err(Into::into),
            )
        })
        .sum()
}
