use std::{cmp::Reverse, num::TryFromIntError};

use itertools::Itertools;

use crate::final_parser;

const INPUT: &str = include_str!("../resources/input11.txt");

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    id: u32,
    pub items: Vec<u32>,
    pub operation: Operation,
    pub test: Test,
    pub inspection_count: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(Argument, Argument),
    Multiply(Argument, Argument),
}
impl Operation {
    fn apply(&self, modulus: u32, old: u32) -> Result<u32, TryFromIntError> {
        match self {
            Self::Add(a, b) => Ok((a.value(old) + b.value(old)) % modulus),
            Self::Multiply(a, b) => ((u64::from(a.value(old)) * u64::from(b.value(old)))
                % u64::from(modulus))
            .try_into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Argument {
    Old,
    Integer(u32),
}
impl Argument {
    const fn value(self, old: u32) -> u32 {
        match self {
            Self::Old => old,
            Self::Integer(i) => i,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Test {
    divisor: u32,
    pass: u32,
    fail: u32,
}
impl Test {
    const fn apply(&self, i: u32) -> u32 {
        if i % self.divisor == 0 {
            self.pass
        } else {
            self.fail
        }
    }
}

pub fn solve_part_a() -> Result<u64, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u64, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u64, anyhow::Error> {
    most_active(input, true, 20)
}

fn part_b(input: &str) -> Result<u64, anyhow::Error> {
    most_active(input, false, 10000)
}

fn most_active(input: &str, reduce_worry: bool, rounds: u32) -> Result<u64, anyhow::Error> {
    let mut monkeys = final_parser(parser::parse)(input)?;
    let common_mod: u32 = monkeys.iter().map(|m| m.test.divisor).unique().product();
    for _ in 0..rounds {
        round(reduce_worry, common_mod, &mut monkeys)?;
    }
    Ok(monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .sorted_by_key(|k| Reverse(*k))
        .take(2)
        .product())
}

fn round(reduce_worry: bool, common_mod: u32, monkeys: &mut [Monkey]) -> Result<(), anyhow::Error> {
    for m_idx in 0..monkeys.len() {
        for i_idx in 0..monkeys[m_idx].items.len() {
            let m = &monkeys[m_idx];
            let mut i = m.items[i_idx];
            i = m.operation.apply(common_mod, i)?;
            if reduce_worry {
                i /= 3;
            }
            let destination = m.test.apply(i);

            monkeys[destination as usize].items.push(i);
        }

        let m = &mut monkeys[m_idx];
        m.inspection_count += m.items.len() as u64;
        m.items.clear();
    }
    Ok(())
}

mod parser {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, u32},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    };

    use super::{Argument, Monkey, Operation, Test};

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
        let monkey = map(
            tuple((
                delimited(tag("Monkey "), u32, pair(tag(":"), line_ending)),
                terminated(items, line_ending),
                terminated(operation, line_ending),
                test,
            )),
            |(id, items, operation, test)| Monkey {
                id,
                items,
                operation,
                test,
                inspection_count: 0,
            },
        );

        separated_list1(pair(line_ending, line_ending), monkey)(input)
    }

    fn items(input: &str) -> IResult<&str, Vec<u32>> {
        preceded(tag("  Starting items: "), separated_list1(tag(", "), u32))(input)
    }
    fn operation(input: &str) -> IResult<&str, Operation> {
        preceded(
            tag("  Operation: new = "),
            alt((
                map(separated_pair(arg, tag(" + "), arg), |(a, b)| {
                    Operation::Add(a, b)
                }),
                map(separated_pair(arg, tag(" * "), arg), |(a, b)| {
                    Operation::Multiply(a, b)
                }),
            )),
        )(input)
    }

    fn arg(input: &str) -> IResult<&str, Argument> {
        alt((
            value(Argument::Old, tag("old")),
            map(u32, Argument::Integer),
        ))(input)
    }

    fn test(input: &str) -> IResult<&str, Test> {
        map(
            tuple((
                delimited(tag("  Test: divisible by "), u32, line_ending),
                delimited(tag("    If true: throw to monkey "), u32, line_ending),
                preceded(tag("    If false: throw to monkey "), u32),
            )),
            |(divisor, pass, fail)| Test {
                divisor,
                pass,
                fail,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example11.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 10605);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 2713310158);
        Ok(())
    }
}
