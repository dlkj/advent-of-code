use anyhow::anyhow;
use anyhow::Context;
use itertools::Itertools;

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &str = "\n\n";

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(include_str!("../resources/input01.txt"))
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(include_str!("../resources/input01.txt"))
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    sum_per_elf(input)?
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("No elfs found!"))
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let answer = sum_per_elf(input)?
        .into_iter()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .sum();

    Ok(answer)
}

fn sum_per_elf(input: &str) -> Result<Vec<u32>, anyhow::Error> {
    input
        .split(DOUBLE_LINE_ENDING)
        .map(|s| s.lines().map(str::parse::<u32>).sum())
        .collect::<Result<Vec<u32>, _>>()
        .context("Failed to parse input text")
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
