use anyhow::{bail, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day18.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let dig_plan = parse_input(input).map_ok(|(dir, dist, _)| (dir, dist));
    let points = dig_plan.process_results(|iter| points(iter))?;
    let area = area(&points);
    Ok(area)
}

fn part2(input: &str) -> Result<usize> {
    let dig_plan = parse_input(input).map(|step| step.and_then(|(_, _, color)| parse_color(color)));
    let points = dig_plan.process_results(|iter| points(iter))?;
    let area = area(&points);
    Ok(area)
}

fn parse_color(color: &str) -> Result<(Direction, usize)> {
    let color = color
        .chars()
        .filter(char::is_ascii_hexdigit)
        .collect::<String>();
    let color = usize::from_str_radix(&color, 16)?;
    let distance = color / 16; //all but last digit
    let direction = color % 16; //last digit
    let direction = match direction {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => bail!("Cannot parse '{}' into direction", direction),
    };
    Ok((direction, distance))
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<(Direction, usize, &str)>> {
    input.lines().map(|line| {
        let Some((direction, distance, color)) = line.split_whitespace().collect_tuple() else {
            bail!("Error parsing line: {}", line)
        };
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => bail!("Unexpected direction: '{}'", direction),
        };
        let distance = distance.parse::<usize>()?;
        Ok((direction, distance, color))
    })
}

fn points(steps: impl Iterator<Item = (Direction, usize)>) -> Vec<(isize, isize)> {
    steps
        .scan((0, 0), |point, (direction, distance)| {
            let distance = distance as isize;
            match direction {
                Direction::Left => point.0 -= distance,
                Direction::Right => point.0 += distance,
                Direction::Up => point.1 += distance,
                Direction::Down => point.1 -= distance,
            };

            Some(*point)
        })
        .collect()
}

fn area(polygon: &[(isize, isize)]) -> usize {
    let inner_area = polygon
        .iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<isize>()
        .unsigned_abs();

    let edges = polygon
        .iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(*x2) + y1.abs_diff(*y2))
        .sum::<usize>();

    (inner_area + edges) / 2 + 1
}
