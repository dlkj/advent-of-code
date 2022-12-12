use std::collections::{BinaryHeap, HashMap};

use anyhow::anyhow;
use itertools::Itertools;

const INPUT: &str = include_str!("../resources/input12.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let (start_x, start_y, _) = input
        .lines()
        .enumerate()
        .flat_map(|(y, r)| r.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| *c == 'S')
        .ok_or_else(|| anyhow::anyhow!("No start marker found"))?;

    let (end_x, end_y, _) = input
        .lines()
        .enumerate()
        .flat_map(|(y, r)| r.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| *c == 'E')
        .ok_or_else(|| anyhow::anyhow!("No end marker found"))?;

    //replace S and E with a and z
    let input = input
        .replace('S', "a")
        .replace('E', "z")
        .lines()
        .map(|l| l.chars().map(u32::from).collect_vec())
        .collect_vec();

    a_star(
        &input,
        (u32::try_from(start_x)?, u32::try_from(start_y)?),
        (u32::try_from(end_x)?, u32::try_from(end_y)?),
    )
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let (end_x, end_y, _) = input
        .lines()
        .enumerate()
        .flat_map(|(y, r)| r.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| *c == 'E')
        .ok_or_else(|| anyhow::anyhow!("No end marker found"))?;

    //replace S and E with a and z
    let input = input
        .replace('S', "a")
        .replace('E', "z")
        .lines()
        .map(|l| l.chars().map(u32::from).collect_vec())
        .collect_vec();

    // process_results(
    input
        .iter()
        .enumerate()
        .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .filter(|(_, _, c)| *c == 'a' as u32)
        // .inspect(|x| println!("{:?}", x))
        .map(|(x, y, _)| {
            a_star(
                &input,
                (u32::try_from(x)?, u32::try_from(y)?),
                (u32::try_from(end_x)?, u32::try_from(end_y)?),
            )
        })
        .filter_map(std::result::Result::ok)
        .min()
        .ok_or_else(|| anyhow!("No routes found"))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Location {
    pub coordinates: (u32, u32),
    pub last: (u32, u32),
    pub steps: u32,
    pub distance: u32,
    pub height: u32,
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (other.steps + other.distance).partial_cmp(&(self.steps + self.distance))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.steps + other.distance).cmp(&(self.steps + self.distance))
    }
}

fn a_star(input: &[Vec<u32>], start: (u32, u32), end: (u32, u32)) -> Result<u32, anyhow::Error> {
    let mut candidates = BinaryHeap::new();

    let mut candidate = Location {
        coordinates: start,
        last: start,
        steps: 0,
        distance: distance(start, end),
        height: input[start.1 as usize][start.0 as usize],
    };

    let mut cheapest = HashMap::new();
    cheapest.insert((0, 0), 0);

    while candidate.coordinates != end {
        // for _ in 0..10 {
        let (x, y) = candidate.coordinates;

        //up
        if x > 0 {
            if let Some(l) = get_step(candidate, (x - 1, y), end, input) {
                replace_or_push(&mut cheapest, &mut candidates, l);
            }
        }

        //down
        if (x as usize) < (input[0].len() - 1) {
            if let Some(l) = get_step(candidate, (x + 1, y), end, input) {
                replace_or_push(&mut cheapest, &mut candidates, l);
            }
        }

        //left
        if y > 0 {
            if let Some(l) = get_step(candidate, (x, y - 1), end, input) {
                replace_or_push(&mut cheapest, &mut candidates, l);
            }
        }

        //right
        if (y as usize) < (input.len() - 1) {
            if let Some(l) = get_step(candidate, (x, y + 1), end, input) {
                replace_or_push(&mut cheapest, &mut candidates, l);
            }
        }

        candidate = candidates
            .pop()
            .ok_or_else(|| anyhow!("No next location candidates left"))?;
    }

    Ok(candidate.steps)
}

fn replace_or_push(
    cheapest: &mut HashMap<(u32, u32), u32>,
    candidates: &mut BinaryHeap<Location>,
    l: Location,
) {
    if let Some(&c) = cheapest.get(&l.coordinates) {
        if l.steps >= c {
            return;
        }
    }
    candidates.push(l);
    cheapest.insert(l.coordinates, l.steps);
}

fn get_step(
    candidate: Location,
    (next_x, next_y): (u32, u32),
    end: (u32, u32),
    input: &[Vec<u32>],
) -> Option<Location> {
    let next_height = input[next_y as usize][next_x as usize];

    if candidate.last != (next_x, next_y) && (candidate.height + 1) >= next_height {
        Some(Location {
            coordinates: (next_x, next_y),
            last: candidate.coordinates,
            steps: candidate.steps + 1,
            distance: distance((next_x, next_y), end),
            height: next_height,
        })
    } else {
        None
    }
}

const fn distance((start_x, start_y): (u32, u32), (end_x, end_y): (u32, u32)) -> u32 {
    // manhatan distance
    start_x.abs_diff(end_x) + start_y.abs_diff(end_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example12.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 31);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 29);
        Ok(())
    }
}
