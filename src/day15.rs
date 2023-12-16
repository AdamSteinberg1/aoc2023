use anyhow::{bail, Result};
use core::array;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day15.txt");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(hash)
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    let steps = input.lines().flat_map(|line| line.split(','));
    let boxes = create_boxes(steps)?;
    let focusing_power = focusing_power(&boxes);
    Ok(focusing_power)
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |current, c| ((current + (c as usize)) * 17) % 256)
}

fn create_boxes<'a>(steps: impl Iterator<Item = &'a str>) -> Result<[Vec<(String, usize)>; 256]> {
    let mut boxes = array::from_fn(|_| vec![]);
    for step in steps {
        let tokens = step.split(['-', '=']).collect::<Vec<_>>();
        match tokens.as_slice() {
            [label, ""] => {
                let index = hash(label);
                boxes[index].retain(|(lens, _)| lens != label);
            }
            &[label, num] => {
                let index = hash(label);
                let lens_box = &mut boxes[index];
                let lens = (label.into(), num.parse()?);
                if let Some(i) = lens_box.iter().position(|(existing, _)| existing == label) {
                    lens_box[i] = lens;
                } else {
                    lens_box.push(lens);
                }
            }
            _ => bail!("Invalid step: {}", step),
        }
    }
    Ok(boxes)
}

fn focusing_power(boxes: &[Vec<(String, usize)>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_num, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(move |(slot, (_, focal_length))| (1 + box_num) * (1 + slot) * focal_length)
        })
        .sum()
}
