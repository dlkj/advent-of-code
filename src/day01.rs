use std::cmp::Reverse;
use std::num::ParseIntError;

use anyhow::anyhow;
use itertools::process_results;
use itertools::Itertools;

use crate::DOUBLE_LINE_ENDING;

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(include_str!("../resources/input01.txt"))
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(include_str!("../resources/input01.txt"))
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    process_results(parse(input), |i| i.max())?.ok_or_else(|| anyhow!("No elfs found!"))
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let results = process_results(parse(input), |sums| {
        sums.sorted_by_key(|k| Reverse(*k)).take(3).sum()
    })?;
    Ok(results)
}

fn parse(input: &str) -> impl Iterator<Item = Result<u32, ParseIntError>> + '_ {
    input.split(DOUBLE_LINE_ENDING).map(parse_and_sum)
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
