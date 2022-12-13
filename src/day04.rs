use crate::day04::parser::parse;
use std::ops::RangeInclusive;

use itertools::{process_results, Itertools};

const INPUT: &str = include_str!("../resources/input04.txt");

pub fn solve_part_a() -> Result<usize, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<usize, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<usize, anyhow::Error> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    process_results(input.lines().map(parse).filter_ok(contains), |i| i.count())
}

fn part_b(input: &str) -> Result<usize, anyhow::Error> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    process_results(input.lines().map(parse).filter_ok(overlaps), |i| i.count())
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
        bytes::complete::tag, character::complete::u32, combinator::map, sequence::separated_pair,
        IResult,
    };

    use crate::final_parser;

    pub(super) fn parse(
        input: &str,
    ) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>), anyhow::Error> {
        final_parser(line)(input)
    }

    fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
        separated_pair(range, tag(","), range)(input)
    }

    fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
        map(separated_pair(u32, tag("-"), u32), |(a, b)| {
            RangeInclusive::new(a, b)
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example04.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 2);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 4);
        Ok(())
    }
}
