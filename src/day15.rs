use itertools::Itertools;
use nom::combinator::iterator;
use rayon::prelude::*;

use crate::finish_parser_it;

const INPUT: &str = include_str!("../resources/input15.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT, 2_000_000)
}

pub fn solve_part_b() -> Result<u64, anyhow::Error> {
    part_b(INPUT, 4_000_000)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    location: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

impl Sensor {
    pub fn new(location: (i32, i32), beacon: (i32, i32)) -> Self {
        Self {
            location,
            beacon,
            distance: (location.0.abs_diff(beacon.0) + location.1.abs_diff(beacon.1))
                .try_into()
                .unwrap_or(i32::MAX),
        }
    }

    pub fn range_at(&self, y: i32) -> Option<(i32, i32)> {
        let h: i32 = self.location.1.abs_diff(y).try_into().unwrap_or(i32::MAX);
        if h > self.distance {
            None
        } else {
            let x = self.location.0;
            Some((x + h - self.distance, x + self.distance - h))
        }
    }
}

fn part_a(input: &str, target_y: i32) -> Result<u32, anyhow::Error> {
    let mut it = iterator(input, parser::parse);

    let sensors = &it.map(|(s, b)| Sensor::new(s, b)).collect_vec();

    let ranges = sensors
        .iter()
        .filter_map(|s| s.range_at(target_y))
        .flat_map(|(s, e)| [(s, 1), (e, -1)])
        .sorted_unstable()
        .group_by(|&(i, _)| i)
        .into_iter()
        .map(|(i, g)| (i, g.map(|(_, c)| c).sum::<i32>()))
        .filter(|(_, s)| *s != 0)
        .collect_vec();

    let mut num = 0;
    let mut start = 0;
    let mut covered = 0;
    for (x, change) in ranges {
        if covered == 0 {
            assert!(change > 0);
            start = x;
        }
        covered += change;
        if covered == 0 {
            assert!(change < 0);
            num += x - start + 1;
            start = 0;
        }
    }

    let becons = sensors
        .iter()
        .filter_map(|s| (s.beacon.1 == target_y).then_some(s.beacon))
        .unique()
        .count();

    finish_parser_it(it)?;
    Ok((num - i32::try_from(becons)?).try_into()?)
}

fn part_b(input: &str, max_val: i32) -> Result<u64, anyhow::Error> {
    let mut it = iterator(input, parser::parse);

    let sensors = &it.map(|(s, b)| Sensor::new(s, b)).collect_vec();

    finish_parser_it(it)?;

    let result = (0..max_val)
        .into_par_iter()
        .filter_map(|target_y| find_beacon(sensors, target_y, max_val))
        .find_first(|_| true);

    let (x, y) = result.ok_or_else(|| anyhow::anyhow!("Not beacon found"))?;
    Ok(u64::try_from(x)? * 4_000_000 + u64::try_from(y)?)
}

fn find_beacon(sensors: &[Sensor], target_y: i32, max_val: i32) -> Option<(i32, i32)> {
    let ranges = sensors
        .iter()
        .filter_map(|s| s.range_at(target_y))
        .flat_map(|(s, e)| [(s, 1), (e, -1)])
        .sorted_unstable()
        .group_by(|&(i, _)| i)
        .into_iter()
        .map(|(i, g)| (i, g.map(|(_, c)| c).sum::<i32>()))
        .filter(|(_, s)| *s != 0)
        .collect_vec();
    let mut num = 0;
    let mut start = 0;
    let mut covered = 0;
    for (x, change) in ranges {
        // println!("x:{} c:{}", x, change);

        if covered == 0 {
            assert!(change > 0);
            start = x;
        }
        covered += change;
        if covered == 0 {
            assert!(change < 0);

            num += x.min(max_val) - start.max(0) + 1;
            start = 0;
        }
    }
    if num == max_val {
        for x in 0..max_val {
            if sensors.iter().all(|s| {
                if let Some((s, e)) = s.range_at(target_y) {
                    !(s <= x && e >= x)
                } else {
                    true
                }
            }) {
                return Some((x, target_y));
            }
        }
        return None;
    }
    None
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::i32,
        character::complete::line_ending,
        combinator::eof,
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    #[allow(clippy::type_complexity)]
    pub(super) fn parse(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
        terminated(
            separated_pair(
                preceded(tag("Sensor at "), coords),
                tag(": closest beacon is at "),
                coords,
            ),
            alt((line_ending, eof)),
        )(input)
    }

    fn coords(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(
            preceded(tag("x="), i32),
            tag(", "),
            preceded(tag("y="), i32),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example15.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE, 10).unwrap(), 26);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE, 20).unwrap(), 56000011);
        Ok(())
    }
}
