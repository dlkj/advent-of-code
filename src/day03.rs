use anyhow::anyhow;
use itertools::{process_results, Itertools};

const INPUT: &str = include_str!("../resources/input03.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    process_results(input.lines().map(find_first_common_between_halves), |i| {
        i.sum()
    })
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    process_results(
        input.lines().tuples().map(find_first_common_to_three),
        |i| i.sum(),
    )
}

fn find_first_common_between_halves(l: &str) -> Result<u32, anyhow::Error> {
    let bitmap = str_to_bitmap(&l[..l.len() / 2])?;
    find_first_common(&l[l.len() / 2..], bitmap)
}

fn find_first_common_to_three((a, b, c): (&str, &str, &str)) -> Result<u32, anyhow::Error> {
    let bitmap = str_to_bitmap(a)? & str_to_bitmap(b)?;
    find_first_common(c, bitmap)
}

fn find_first_common(s: &str, bitmap: u64) -> Result<u32, anyhow::Error> {
    for c in s.chars() {
        let char_idx = char_idx(c)?;
        if bitmap & (1u64 << char_idx) > 0 {
            return Ok(char_idx + 1);
        }
    }
    Err(anyhow!("No common item found"))
}

fn str_to_bitmap(l: &str) -> Result<u64, anyhow::Error> {
    l.chars()
        .map(char_idx)
        .fold_ok(0u64, |acc, i| acc | (1u64 << i))
}

fn char_idx(c: char) -> Result<u32, anyhow::Error> {
    if ('a'..='z').contains(&c) {
        Ok(c as u32 - 'a' as u32)
    } else if ('A'..='Z').contains(&c) {
        Ok(c as u32 - 'A' as u32 + 26)
    } else {
        Err(anyhow!("char out of range"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example03.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 157);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 70);
        Ok(())
    }
}
