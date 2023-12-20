use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use ranges::{self, GenericRange, OperationResult};
use std::{
    collections::HashMap,
    ops::{Bound, RangeBounds},
};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day19.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u64> {
    let (workflows, parts) = input.split_once("\n\n").context("Error parsing input")?;
    let workflows = parse_workflows(workflows)?;
    let parts = parse_parts(parts);

    parts
        .filter_ok(|part| is_part_accepted(part, &workflows))
        .map_ok(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn part2(input: &str) -> Result<u64> {
    let (workflows, _) = input.split_once("\n\n").context("Error parsing input")?;
    let workflows = parse_workflows(workflows)?;
    let ranges = PartRanges {
        x: (1..=4000).into(),
        m: (1..=4000).into(),
        a: (1..=4000).into(),
        s: (1..=4000).into(),
    };
    Ok(count_accepted(ranges, "in", &workflows))
}

fn is_part_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut output = workflows["in"].process(part);
    loop {
        match output {
            Output::Accept => return true,
            Output::Reject => return false,
            Output::Workflow(next) => output = workflows[&next].process(part),
        }
    }
}

fn parse_parts(parts: &str) -> impl Iterator<Item = Result<Part>> + '_ {
    parts.lines().map(|line| {
        let nums = line
            .split(',')
            .map(|category| {
                category
                    .split(|c: char| !c.is_ascii_digit())
                    .find(|s| !s.is_empty())
                    .context("Missing rating")?
                    .parse()
                    .map_err(Into::into)
            })
            .collect::<Result<Vec<_>>>()?;
        match nums.as_slice() {
            &[x, m, a, s] => Ok(Part { x, m, a, s }),
            _ => bail!("Expected 4 ratings for part. Actual={:?}", nums),
        }
    })
}

fn count_accepted(
    mut ranges: PartRanges,
    workflow_name: &str,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    let workflow = &workflows[workflow_name];
    workflow
        .rules
        .iter()
        .map(|rule| {
            if let Some(category) = rule.category {
                let intersection = rule.range & ranges.get(category);
                let count = match intersection {
                    OperationResult::Single(intersection) => {
                        let mut ranges = ranges.clone();
                        *ranges.get_mut(category) = intersection;
                        match &rule.output {
                            Output::Accept => ranges.count(),
                            Output::Reject => 0,
                            Output::Workflow(name) => count_accepted(ranges, name, workflows),
                        }
                    }
                    OperationResult::Empty => 0,
                    _ => unreachable!(),
                };

                let range = ranges.get_mut(category);
                *range = match range.difference(rule.range) {
                    OperationResult::Empty => GenericRange::new_less_than(0), //sneaky way to make empty range
                    OperationResult::Single(difference) => difference,
                    _ => panic!("Unexpected split into two ranges"),
                };

                count
            } else {
                match &rule.output {
                    Output::Accept => ranges.count(),
                    Output::Reject => 0,
                    Output::Workflow(name) => count_accepted(ranges.clone(), name, workflows),
                }
            }
        })
        .sum()
}

#[derive(Clone)]
enum Output {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        match value {
            "A" => Output::Accept,
            "R" => Output::Reject,
            dest => Output::Workflow(dest.to_string()),
        }
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone)]
struct PartRanges {
    x: GenericRange<u64>,
    m: GenericRange<u64>,
    a: GenericRange<u64>,
    s: GenericRange<u64>,
}

fn size(range: GenericRange<u64>) -> u64 {
    let start = match range.start_bound() {
        Bound::Included(&num) => num,
        Bound::Excluded(&num) => num + 1,
        Bound::Unbounded => return u64::MAX,
    };

    let end = match range.end_bound() {
        Bound::Included(&num) => num,
        Bound::Excluded(&num) => num - 1,
        Bound::Unbounded => return u64::MAX,
    };

    1 + end - start
}

impl PartRanges {
    fn get(&self, category: Category) -> GenericRange<u64> {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn get_mut(&mut self, category: Category) -> &mut GenericRange<u64> {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    fn count(&self) -> u64 {
        [self.x, self.m, self.a, self.s]
            .into_iter()
            .map(size)
            .product()
    }
}

impl Part {
    fn get(&self, category: Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> Output {
        for rule in &self.rules {
            if let Some(category) = rule.category {
                if rule.range.contains(&part.get(category)) {
                    return rule.output.clone();
                }
            } else {
                return rule.output.clone();
            }
        }

        panic!()
    }
}

#[derive(Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Unexpected category: {}", value),
        }
    }
}

struct Rule {
    range: GenericRange<u64>,
    category: Option<Category>,
    output: Output,
}

fn parse_workflows(workflows: &str) -> Result<HashMap<String, Workflow>> {
    workflows
        .lines()
        .map(|line| {
            let (name, remaining) = line
                .split_once('{')
                .context(anyhow!("Error parsing workflow name from {}", line))?;

            let rules = parse_rules(remaining)?;

            let workflow = Workflow { rules };

            Ok((name.to_string(), workflow))
        })
        .collect()
}

fn parse_rules(remaining: &str) -> Result<Vec<Rule>> {
    remaining
        .split(',')
        .map(|token| {
            if let Some((condition, output)) = token.split_once(':') {
                let mut condition = condition.chars();
                let category = condition.next(); //can be one of x,m,a,s
                let cmp = condition.next(); // < or >
                let (category, cmp) = category
                    .zip(cmp)
                    .context(anyhow!("Error parsing rule: {}", token))?;
                let num = condition.as_str().parse()?;

                let range = match cmp {
                    '<' => GenericRange::new_less_than(num),
                    '>' => GenericRange::new_greater_than(num),
                    _ => bail!("Unexpected operator: {}", cmp),
                };
                Ok(Rule {
                    range,
                    category: Some(category.into()),
                    output: output.into(),
                })
            } else {
                let token = token.replace('}', "");
                Ok(Rule {
                    range: (..).into(),
                    category: None,
                    output: token.as_str().into(),
                })
            }
        })
        .collect()
}
