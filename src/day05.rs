use crate::{
    day05::parser::{parse_instructions, parse_locations},
    DOUBLE_LINE_ENDING,
};

use anyhow::anyhow;
use itertools::Itertools;

const INPUT: &str = include_str!("../resources/input05.txt");

#[derive(Debug, Clone, Copy)]
struct Crate(char);
impl From<Crate> for char {
    fn from(c: Crate) -> Self {
        c.0
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

pub fn solve_part_a() -> Result<String, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<String, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<String, anyhow::Error> {
    let (mut locations, instructions) = parse_input(input)?;

    for i in instructions {
        for _ in 0..i.count {
            let c = locations[i.from - 1]
                .pop()
                .ok_or_else(|| anyhow!("Tried to pop an empty stack of crates"))?;
            locations[i.to - 1].push(c);
        }
    }

    Ok(read_top_crates(locations))
}

fn part_b(input: &str) -> Result<String, anyhow::Error> {
    let (mut locations, instructions) = parse_input(input)?;

    for i in instructions {
        let from = &mut locations[i.from - 1];
        let moving_crates = from.drain(from.len() - i.count..).collect_vec();
        locations[i.to - 1].extend(moving_crates);
    }

    Ok(read_top_crates(locations))
}

fn read_top_crates(locations: Vec<Vec<Crate>>) -> String {
    locations
        .into_iter()
        .map(|l| Into::<char>::into(l[l.len() - 1]))
        .collect()
}

fn parse_input(input: &str) -> Result<(Vec<Vec<Crate>>, Vec<Instruction>), anyhow::Error> {
    let (locations_input, instructions_input) = input
        .split(DOUBLE_LINE_ENDING)
        .next_tuple()
        .ok_or_else(|| anyhow!("unable to read input and location section from input"))?;

    Ok((
        parse_locations(locations_input)?,
        parse_instructions(instructions_input)?,
    ))
}

mod parser {
    use anyhow::anyhow;
    use nom::branch::alt;
    use nom::character::complete::satisfy;
    use nom::combinator::{map, value};
    use nom::multi::separated_list1;
    use nom::sequence::tuple;
    use nom::AsChar;
    use nom::{bytes::complete::tag, sequence::delimited, IResult};

    use crate::{dec_int, final_parser};

    use super::{Crate, Instruction};

    pub(super) fn parse_locations(input: &str) -> Result<Vec<Vec<Crate>>, anyhow::Error> {
        let mut lines = input.lines().rev();

        let stack_numbers_input = lines
            .next()
            .ok_or_else(|| anyhow!("unable to read location stack numbers input"))?;

        let mut stacks = Vec::new();
        stacks.extend(
            final_parser(stack_numbers)(stack_numbers_input)?
                .into_iter()
                .map(|_| Vec::new()),
        );

        for line in lines {
            let crates = final_parser(crates)(line)?;

            for c in crates.into_iter().enumerate() {
                if let (i, Some(c)) = c {
                    stacks[i].push(c);
                }
            }
        }
        Ok(stacks)
    }

    fn stack_numbers(input: &str) -> IResult<&str, Vec<char>> {
        separated_list1(
            tag(" "),
            delimited(tag(" "), satisfy(AsChar::is_dec_digit), tag(" ")),
        )(input)
    }

    fn crates(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
        separated_list1(
            tag(" "),
            alt((
                value(None, tag("   ")),
                map(
                    delimited(tag("["), satisfy(AsChar::is_alpha), tag("]")),
                    |c| Some(Crate(c)),
                ),
            )),
        )(input)
    }

    pub(super) fn parse_instructions(input: &str) -> Result<Vec<Instruction>, anyhow::Error> {
        input
            .lines()
            .map(|l| final_parser(instruction)(l))
            .collect::<Result<Vec<_>, _>>()
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        let (input, (_, count, _, from, _, to)) = tuple((
            tag("move "),
            dec_int,
            tag(" from "),
            dec_int,
            tag(" to "),
            dec_int,
        ))(input)?;

        Ok((input, Instruction { count, from, to }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example05.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE)?, "CMZ");
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE)?, "MCD");
        Ok(())
    }
}
