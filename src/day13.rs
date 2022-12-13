use std::{cmp::Ordering, fmt::Debug};

use itertools::Itertools;
use nom::combinator::iterator;

use crate::finish_parser_it;

const INPUT: &str = include_str!("../resources/input13.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let mut it = iterator(input, parser::parse_pairs);

    let result = it
        .map(|(a, b)| a.cmp(&b))
        .enumerate()
        .filter_map(|(a, b)| {
            if b == Ordering::Less {
                Some(a + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    finish_parser_it(it)?;
    Ok(result.try_into()?)
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let mut it = iterator(input, parser::parse_individual);

    let div1 = vec![PacketItem::List(vec![PacketItem::Integer(2)])];
    let div2 = vec![PacketItem::List(vec![PacketItem::Integer(6)])];

    let sorted = it
        .chain([div1.clone(), div2.clone()].into_iter())
        .sorted_unstable()
        .collect_vec();

    finish_parser_it(it)?;
    let result = (sorted.binary_search(&div1).unwrap_or(0) + 1)
        * (sorted.binary_search(&div2).unwrap_or(0) + 1);
    Ok(result.try_into()?)
}

#[derive(PartialEq, Eq, Clone)]
enum PacketItem {
    List(Vec<Self>),
    Integer(u32),
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Integer(b)) => a.cmp(&vec![Self::Integer(*b)]),
            (Self::Integer(a), Self::List(b)) => vec![Self::Integer(*a)].cmp(b),
        }
    }
}

impl Debug for PacketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(l) => l.fmt(f),
            Self::Integer(i) => std::fmt::Debug::fmt(&i, f),
        }
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, u32},
        combinator::{eof, map, recognize},
        multi::separated_list0,
        sequence::{delimited, pair, terminated},
        IResult,
    };

    use super::PacketItem;

    pub(super) fn parse_pairs(input: &str) -> IResult<&str, (Vec<PacketItem>, Vec<PacketItem>)> {
        terminated(
            pair(terminated(packet, line_ending), packet),
            alt((recognize(pair(line_ending, line_ending)), recognize(eof))),
        )(input)
    }

    pub(super) fn parse_individual(input: &str) -> IResult<&str, Vec<PacketItem>> {
        terminated(
            packet,
            alt((
                recognize(pair(line_ending, line_ending)),
                recognize(line_ending),
                recognize(eof),
            )),
        )(input)
    }

    fn packet(input: &str) -> IResult<&str, Vec<PacketItem>> {
        delimited(
            tag("["),
            separated_list0(
                tag(","),
                alt((map(packet, PacketItem::List), map(u32, PacketItem::Integer))),
            ),
            tag("]"),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example13.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 13);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 140);
        Ok(())
    }
}
