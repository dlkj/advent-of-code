use itertools::{repeat_n, Itertools};

const INPUT: &str = include_str!("../resources/input09.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn solve_part_a() -> Result<usize, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<usize, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<usize, anyhow::Error> {
    let result = parser::parse(input)?
        .into_iter()
        .flat_map(|(d, c)| repeat_n(d, c as usize))
        // head locations
        .scan((0, 0), scan_head)
        // tail locations
        .scan((0, 0), scan_tail)
        // drop duplicates
        .unique()
        .count();

    Ok(result)
}

fn part_b(input: &str) -> Result<usize, anyhow::Error> {
    let result = parser::parse(input)?
        .into_iter()
        .flat_map(|(d, c)| repeat_n(d, c as usize))
        // head locations
        .scan((0, 0), scan_head)
        // tail locations for 9 other knots
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        .scan((0, 0), scan_tail)
        // drop duplicates
        .unique()
        .count();

    Ok(result)
}

#[allow(clippy::unnecessary_wraps)]
fn scan_head((x, y): &mut (i32, i32), d: Direction) -> Option<(i32, i32)> {
    match d {
        Direction::Right => *x += 1,
        Direction::Left => *x -= 1,
        Direction::Up => *y += 1,
        Direction::Down => *y -= 1,
    };

    Some((*x, *y))
}

#[allow(clippy::unnecessary_wraps)]
fn scan_tail((tx, ty): &mut (i32, i32), (hx, hy): (i32, i32)) -> Option<(i32, i32)> {
    if !((-1..=1).contains(&(*tx - hx)) && (-1..=1).contains(&(*ty - hy))) {
        *tx -= num::clamp(*tx - hx, -1, 1);
        *ty -= num::clamp(*ty - hy, -1, 1);
    }
    Some((*tx, *ty))
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, u32},
        combinator::value,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use crate::final_parser;

    use super::Direction;

    pub(super) fn parse(input: &str) -> Result<Vec<(Direction, u32)>, anyhow::Error> {
        final_parser(separated_list1(line_ending, line))(input)
    }

    fn line(input: &str) -> IResult<&str, (Direction, u32)> {
        separated_pair(direction, tag(" "), u32)(input)
    }

    fn direction(input: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example09.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 13);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 1);
        Ok(())
    }
}
