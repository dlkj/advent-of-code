use std::cmp::Reverse;
use std::num::ParseIntError;

use anyhow::anyhow;
use anyhow::Context;

use crate::DOUBLE_LINE_ENDING;

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(include_str!("../resources/input01.txt"))
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(include_str!("../resources/input01.txt"))
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    parse(input)?
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("No elfs found!"))
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let mut sums = parse(input)?;
    sums.sort_by_key(|k| Reverse(*k));
    Ok(sums.into_iter().take(3).sum())
}

fn parse(input: &str) -> Result<Vec<u32>, anyhow::Error> {
    input
        .split(DOUBLE_LINE_ENDING)
        .map(parse_and_sum)
        .collect::<Result<Vec<u32>, _>>()
        .context("Failed to parse input text")
}

fn parse_and_sum(s: &str) -> Result<u32, ParseIntError> {
    s.lines().map(str::parse::<u32>).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(include_str!("../resources/example01.txt"))?, 24000);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(include_str!("../resources/example01.txt"))?, 45000);
        Ok(())
    }
}
