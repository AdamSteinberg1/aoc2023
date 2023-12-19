use anyhow::{anyhow, Context, Result};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() -> Result<()> {
    let input = include_str!("../inputs/day17.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let city = parse_input(input)?;
    minimum_heat_loss(3, 0, &city)
}

fn part2(input: &str) -> Result<usize> {
    let city = parse_input(input)?;
    minimum_heat_loss(10, 4, &city)
}

fn parse_input(input: &str) -> Result<Vec<Vec<usize>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|num| num as usize)
                        .context(anyhow!("Error parsing '{}' into number", c))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::North, Self::East, Self::South, Self::West].into_iter()
    }

    fn opposite(&self) -> Self {
        match self {
            Self::East => Self::West,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

fn get_neighbor(
    position: (usize, usize),
    direction: Direction,
    city: &[Vec<usize>],
) -> Option<(usize, usize)> {
    let (row, col) = position;
    let (row, col) = match direction {
        Direction::North => (row.checked_sub(1)?, col),
        Direction::South => (row + 1, col),
        Direction::West => (row, col.checked_sub(1)?),
        Direction::East => (row, col + 1),
    };
    city.get(row)?.get(col)?;
    Some((row, col))
}

fn get_neighbors(
    position: (usize, usize),
    city: &[Vec<usize>],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    Direction::iter().filter_map(move |dir| get_neighbor(position, dir, city))
}

fn manhattan_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

//the direction you would travel moving from src to dest
//panics if src and dest are not adjacent
fn get_direction(src: (usize, usize), dest: (usize, usize)) -> Direction {
    let difference = (
        dest.0 as isize - src.0 as isize,
        dest.1 as isize - src.1 as isize,
    );
    match difference {
        (-1, 0) => Direction::North,
        (0, 1) => Direction::East,
        (1, 0) => Direction::South,
        (0, -1) => Direction::West,
        _ => panic!("Error: {:?} and {:?} must be adjacent", src, dest),
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Node {
    position: (usize, usize),
    direction: Direction,
    count: usize, //number of moves in the current direction
}

//Pathfinds from top-left to bottom-right, adding up heats along the way
//Uses A* Algorithm https://en.wikipedia.org/wiki/A*_search_algorithm
fn minimum_heat_loss(
    max_colinear_moves: usize,
    min_colinear_moves: usize,
    city: &[Vec<usize>],
) -> Result<usize> {
    let start = (0, 0);
    let goal = (city.len() - 1, city[0].len() - 1);

    let heuristic = |node: (usize, usize)| manhattan_distance(node, goal);

    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    let mut open_set = BinaryHeap::new();

    // For node n, g[n] is the cost of the cheapest path from start to n currently known.
    let mut costs = HashMap::new();

    // f(n) = g(n) + h(n)
    // f(n) represents our current best guess as to how cheap a path could be from start to finish if it goes through n.
    // g(n) = 0 for the start node
    let f = Reverse(heuristic(start));

    //We have two possible start nodes

    // Start node assuming the first move is east
    let start_node = Node {
        position: start,
        direction: Direction::East,
        count: 0,
    };

    open_set.push((f, start_node.clone()));
    costs.insert(start_node, 0);

    // Start node assuming the first move is south
    let start_node = Node {
        position: start,
        direction: Direction::South,
        count: 0,
    };
    open_set.push((f, start_node.clone()));
    costs.insert(start_node, 0);

    //get the node with the lowest f(n)
    while let Some((_, current)) = open_set.pop() {
        if current.position == goal && current.count >= min_colinear_moves {
            //We found the goal!
            if let Some((_, &min_heat)) = costs.iter().find(|(node, _)| node.position == goal) {
                return Ok(min_heat);
            }
        }

        for neighbor in get_neighbors(current.position, city) {
            let direction = get_direction(current.position, neighbor);

            //skip if too many steps in the same direction
            if current.direction == direction && current.count >= max_colinear_moves {
                continue;
            }

            //skip if too few steps in the same direction
            if current.direction != direction && current.count < min_colinear_moves {
                continue;
            }

            //skip if turning around
            if current.direction == direction.opposite() {
                continue;
            }

            //count how many moves we've traveled in the same direction
            let count = if direction == current.direction {
                current.count + 1
            } else {
                1
            };
            let neighbor_node = Node {
                position: neighbor,
                direction,
                count,
            };

            let heat = city[neighbor.0][neighbor.1];
            //the distance from start to the neighbor through current
            let g = costs[&current] + heat;

            if &g >= costs.get(&neighbor_node).unwrap_or(&usize::MAX) {
                //This path to neighbor is not better than any previous one. Skip it!
                continue;
            }

            //This path to neighbor is better than any previous one. Record it!
            costs.insert(neighbor_node.clone(), g);

            let h = heuristic(neighbor);
            let f = g + h;

            //if neighbor not in open_set
            if open_set.iter().all(|(_, node)| node != &neighbor_node) {
                open_set.push((Reverse(f), neighbor_node));
            }
        }
    }

    // Open set is empty but goal was never reached
    Err(anyhow!("Could not find path to goal"))
}
