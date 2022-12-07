use std::collections::VecDeque;

use anyhow::anyhow;
use itertools::Itertools;

const INPUT: &str = include_str!("../resources/input06.txt");

pub fn solve_part_a() -> Result<usize, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<usize, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<usize, anyhow::Error> {
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .find(|(_, (a, b, c, d))| [a, b, c, d].iter().all_unique())
        .map(|(i, _)| i + 4)
        .ok_or_else(|| anyhow!("No range found"))
}

fn part_b(input: &str) -> Result<usize, anyhow::Error> {
    let mut buff = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        buff.push_back(c);

        if buff.len() > 14 {
            buff.pop_front();
        }
        if buff.len() == 14 && buff.iter().all_unique() {
            return Ok(i + 1);
        }
    }
    Err(anyhow!("No range found"))
}
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example06.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE)?, 7);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE)?, 19);
        Ok(())
    }
}
