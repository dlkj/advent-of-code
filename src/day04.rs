use crate::day04::parser::parse;
use std::ops::RangeInclusive;

use itertools::Itertools;

const INPUT: &str = include_str!("../resources/input04.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let a = input
        .lines()
        .map(parse)
        .filter_ok(contains)
        .fold_ok(0, |a, _| a + 1)?;

    Ok(a)
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let a = input
        .lines()
        .map(parse)
        .filter_ok(overlaps)
        .fold_ok(0, |a, _| a + 1)?;

    Ok(a)
}

fn contains((a, b): &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    if a.end() - a.start() > b.end() - b.start() {
        a.start() <= b.start() && a.end() >= b.end()
    } else {
        b.start() <= a.start() && b.end() >= a.end()
    }
}

fn overlaps((a, b): &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    a.end() >= b.start() && a.start() <= b.end()
}

mod parser {
    use std::ops::RangeInclusive;

    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        error::Error,
        sequence::separated_pair,
        IResult,
    };
    use nom_supreme::final_parser::final_parser;

    pub(super) fn parse(
        input: &str,
    ) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>), Error<String>> {
        final_parser(line)(input).map_err(|e: Error<&str>| Error::new(e.input.to_owned(), e.code))
    }

    fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
        separated_pair(range, tag(","), range)(input)
    }

    fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
        map(separated_pair(u32_dec, tag("-"), u32_dec), |(a, b)| {
            RangeInclusive::new(a, b)
        })(input)
    }

    fn u32_dec(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example04.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE)?, 2);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE)?, 4);
        Ok(())
    }
}
