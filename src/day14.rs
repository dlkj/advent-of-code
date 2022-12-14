use itertools::Itertools;
use nom::combinator::iterator;

use crate::finish_parser_it;

const INPUT: &str = include_str!("../resources/input14.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let mut lines = iterator(input, parser::parse);

    let mut cave = vec![[Tile::Empty; 1000]; 1000];

    for l in &mut lines {
        for (a, b) in l.into_iter().tuple_windows() {
            draw_line(&mut cave, a, b)?;
        }
    }

    let max_depth = max_depth(&cave)
        .ok_or_else(|| anyhow::anyhow!("No rocks found"))?
        .try_into()?;

    loop {
        let (mut sand_x, mut sand_y) = (500, 0);
        loop {
            if let Some(next) = get_next_empty_tile(&cave, (sand_x, sand_y)) {
                (sand_x, sand_y) = next;
            } else {
                cave[sand_y as usize][sand_x as usize] = Tile::Sand;
                break;
            }

            if sand_y > max_depth {
                break;
            }
        }
        if sand_y > max_depth {
            break;
        }
    }

    finish_parser_it(lines)?;
    Ok(cave
        .into_iter()
        .map(|r| r.iter().filter(|&&t| t == Tile::Sand).count())
        .sum::<usize>()
        .try_into()?)
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let mut lines = iterator(input, parser::parse);

    let mut cave = vec![[Tile::Empty; 1000]; 1000];

    for l in &mut lines {
        for (a, b) in l.into_iter().tuple_windows() {
            draw_line(&mut cave, a, b)?;
        }
    }

    let max_d: u32 = max_depth(&cave)
        .ok_or_else(|| anyhow::anyhow!("No rocks found"))?
        .try_into()?;

    draw_line(&mut cave, (0, max_d + 2), (999, max_d + 2))?;

    let max_d: u32 = max_depth(&cave)
        .ok_or_else(|| anyhow::anyhow!("No rocks found"))?
        .try_into()?;

    loop {
        let (mut sand_x, mut sand_y) = (500, 0);
        loop {
            if let Some(next) = get_next_empty_tile(&cave, (sand_x, sand_y)) {
                (sand_x, sand_y) = next;
            } else {
                cave[sand_y as usize][sand_x as usize] = Tile::Sand;
                break;
            }

            if sand_y > max_d + 2 {
                return Err(anyhow::anyhow!("Sand escape!"));
            }
        }

        if (sand_x, sand_y) == (500, 0) {
            break;
        }
        if sand_y > max_d + 2 {
            return Err(anyhow::anyhow!("Sand escape!"));
        }
    }

    finish_parser_it(lines)?;
    Ok(cave
        .into_iter()
        .map(|r| r.iter().filter(|&&t| t == Tile::Sand).count())
        .sum::<usize>()
        .try_into()?)
}

fn max_depth(cave: &[[Tile; 1000]]) -> Option<usize> {
    cave.iter()
        .enumerate()
        .filter_map(|(i, &l)| l.iter().any(|&t| t == Tile::Rock).then_some(i))
        .max()
}

fn get_next_empty_tile(cave: &[[Tile; 1000]], (sand_x, sand_y): (u32, u32)) -> Option<(u32, u32)> {
    if cave[(sand_y + 1) as usize][sand_x as usize] == Tile::Empty {
        Some((sand_x, sand_y + 1))
    } else if cave[(sand_y + 1) as usize][(sand_x - 1) as usize] == Tile::Empty {
        Some((sand_x - 1, sand_y + 1))
    } else if cave[(sand_y + 1) as usize][(sand_x + 1) as usize] == Tile::Empty {
        Some((sand_x + 1, sand_y + 1))
    } else {
        None
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

fn draw_line(
    cave: &mut [[Tile; 1000]],
    (a_x, a_y): (u32, u32),
    (b_x, b_y): (u32, u32),
) -> Result<(), anyhow::Error> {
    if a_x == b_x {
        for y in a_y.min(b_y)..=a_y.max(b_y) {
            cave[y as usize][a_x as usize] = Tile::Rock;
        }
    } else if a_y == b_y {
        for x in a_x.min(b_x)..=a_x.max(b_x) {
            cave[a_y as usize][x as usize] = Tile::Rock;
        }
    } else {
        return Err(anyhow::anyhow!("can't draw diagonal lines"));
    }

    Ok(())
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::{complete::line_ending, complete::u32},
        combinator::eof,
        multi::separated_list1,
        sequence::{separated_pair, terminated},
        IResult,
    };

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
        terminated(
            separated_list1(tag(" -> "), separated_pair(u32, tag(","), u32)),
            alt((line_ending, eof)),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example14.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 24);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 93);
        Ok(())
    }
}
