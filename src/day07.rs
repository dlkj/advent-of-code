use anyhow::anyhow;

use self::parser::parse;

const INPUT: &str = include_str!("../resources/input07.txt");

#[derive(Debug)]
enum TerminalLine<'a> {
    ChangeDirectory(&'a str),
    List(Vec<ListLine<'a>>),
}

#[derive(Debug, Clone, Copy)]
enum ListLine<'a> {
    File(usize, &'a str),
    Directory(&'a str),
}

pub fn solve_part_a() -> Result<usize, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<usize, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<usize, anyhow::Error> {
    let sizes = get_dir_sizes(input)?;

    Ok(sizes.into_iter().filter(|&s| s <= 100_000).sum())
}

fn part_b(input: &str) -> Result<usize, anyhow::Error> {
    let sizes = get_dir_sizes(input)?;

    let to_free = sizes[sizes.len() - 1] - 40_000_000;

    Ok(sizes
        .into_iter()
        .fold(usize::MAX, |a, b| if b > to_free { a.min(b) } else { a }))
}

fn get_dir_sizes(input: &str) -> Result<Vec<usize>, anyhow::Error> {
    let lines = parse(input)?;
    let mut sizes = Vec::new();
    let mut current_total = 0;
    let mut total_stack = Vec::new();
    for l in lines {
        match l {
            TerminalLine::ChangeDirectory(d) => match d {
                "/" => {
                    //slightly naughty assumption that this is only present at the start of the input
                }
                ".." => {
                    sizes.push(current_total);
                    current_total += total_stack
                        .pop()
                        .ok_or_else(|| anyhow!("Tried to pop empty stack"))?;
                }
                _ => {
                    total_stack.push(current_total);
                    current_total = 0;
                }
            },
            TerminalLine::List(list_items) => {
                current_total += file_size_sum(list_items);
            }
        }
    }

    //and the current dir
    sizes.push(current_total);

    //and unwind the stack back to the root
    for t in total_stack {
        current_total += t;
        sizes.push(current_total);
    }
    Ok(sizes)
}

fn file_size_sum(list_items: Vec<ListLine>) -> usize {
    list_items
        .into_iter()
        .map(|i| match i {
            ListLine::File(s, _) => s,
            ListLine::Directory(_) => 0,
        })
        .sum()
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, not_line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    use crate::{dec_int, final_parser};

    use super::{ListLine, TerminalLine};

    pub(super) fn parse(input: &str) -> Result<Vec<TerminalLine>, anyhow::Error> {
        final_parser(separated_list1(line_ending, terminal_line))(input)
    }

    fn terminal_line(input: &str) -> IResult<&str, TerminalLine> {
        alt((
            map(change_directory, TerminalLine::ChangeDirectory),
            map(list, TerminalLine::List),
        ))(input)
    }

    fn change_directory(input: &str) -> IResult<&str, &str> {
        preceded(tag("$ cd "), not_line_ending)(input)
    }

    fn list(input: &str) -> IResult<&str, Vec<ListLine>> {
        preceded(
            terminated(tag("$ ls"), line_ending),
            separated_list1(
                line_ending,
                alt((
                    map(directory, ListLine::Directory),
                    map(file, |(size, name)| ListLine::File(size, name)),
                )),
            ),
        )(input)
    }

    fn directory(input: &str) -> IResult<&str, &str> {
        preceded(tag("dir "), not_line_ending)(input)
    }
    fn file(input: &str) -> IResult<&str, (usize, &str)> {
        separated_pair(dec_int, tag(" "), not_line_ending)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example07.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE)?, 95437);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE)?, 24933642);
        Ok(())
    }
}
