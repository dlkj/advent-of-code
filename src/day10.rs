use std::vec;

use itertools::process_results;
use nom::{combinator::iterator, Parser};

const INPUT: &str = include_str!("../resources/input10.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i32),
}

pub fn solve_part_a() -> Result<i32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<String, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<i32, anyhow::Error> {
    let mut instructions = iterator(input, parser::line);

    let result = process_results(
        register_values(&mut instructions)
            .enumerate()
            .map(|(i, x)| Ok(i32::try_from(1 + i)? * x))
            .skip(19)
            .step_by(40),
        #[allow(clippy::redundant_closure_for_method_calls)]
        |i| i.sum(),
    );

    finish(instructions)?;

    result
}

fn part_b(input: &str) -> Result<String, anyhow::Error> {
    let mut instructions = iterator(input, parser::line);

    let result = process_results(
        register_values(&mut instructions)
            .enumerate()
            .map(|(i, x)| {
                let n = i32::try_from(i % 40)?;

                Ok(if n != 39 {
                    if (n - 1..=n + 1).contains(&x) {
                        "#"
                    } else {
                        "."
                    }
                } else if (n - 1..=n + 1).contains(&x) {
                    "#\n"
                } else {
                    ".\n"
                })
            }),
        #[allow(clippy::redundant_closure_for_method_calls)]
        |i| i.collect(),
    );

    finish(instructions)?;

    result
}

fn register_values(input: impl Iterator<Item = Instruction>) -> impl Iterator<Item = i32> {
    input
        .flat_map(|i| match i {
            Instruction::Noop => vec![i],
            Instruction::AddX(_) => vec![Instruction::Noop, i],
        })
        .scan(1, |x, i| {
            let current = *x;
            match i {
                Instruction::Noop => {}
                Instruction::AddX(a) => {
                    *x += a;
                }
            };
            Some(current)
        })
}

fn finish<'a, O, F>(
    parse_it: nom::combinator::ParserIterator<&'a str, nom::error::Error<&'a str>, F>,
) -> Result<(), anyhow::Error>
where
    F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    let (input, _) = parse_it
        .finish()
        .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;

    if input.trim().is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("unparsed data"))
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::{eof, map, value},
        sequence::{preceded, terminated},
        IResult,
    };

    use crate::dec_iint;

    use super::Instruction;

    pub(super) fn line(input: &str) -> IResult<&str, Instruction> {
        terminated(instruction, alt((line_ending, eof)))(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        let noop = value(Instruction::Noop, tag("noop"));
        let addx = map(preceded(tag("addx "), dec_iint), Instruction::AddX);

        alt((noop, addx))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example10.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 13140);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(
            part_b(EXAMPLE).unwrap(),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
        Ok(())
    }
}
