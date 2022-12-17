use std::collections::HashMap;

use itertools::Itertools;

use crate::final_parser;

const INPUT: &str = include_str!("../resources/input16.txt");

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let solutions = calc_solutions(input, 30)?;
    Ok(solutions.values().copied().max().unwrap_or(0))
}

fn calc_solutions(
    input: &str,
    max_time: usize,
) -> Result<HashMap<(&str, u64), u32>, anyhow::Error> {
    let valves = final_parser(parser::parse)(input)?;
    let valves: HashMap<_, _> = valves.into_iter().map(|v| (v.name, v)).collect();
    let v_idx: HashMap<_, _> = valves.keys().enumerate().map(|(i, &v)| (v, i)).collect();
    let flows: HashMap<_, _> = v_idx
        .iter()
        .map(|(&s, &i)| (1u64 << i, valves[s].flow_rate))
        .collect();
    let mut max_preasure = HashMap::new();
    let v = &valves["AA"];
    max_preasure.insert((v.name, 1u64 << v_idx[v.name]), 0);
    for &t in &v.tunnels {
        max_preasure.insert((t, 0), 0);
    }
    for _ in 0..(max_time - 1) {
        let mut next_max_preasure: HashMap<(&str, u64), u32> = HashMap::new();
        for ((v_name, opened), presure) in max_preasure {
            let v = &valves[v_name];
            let next_presure = flows
                .iter()
                .map(|(&i, &f)| if i & opened == 0 { 0 } else { f })
                .sum::<u32>()
                + presure;

            let next_mp = &mut next_max_preasure;

            // turn on valve
            let mask = 1 << v_idx[v.name];
            if v.flow_rate > 0 && opened & mask == 0 {
                //needs to be max
                let k = (v_name, opened | mask);
                next_mp.insert(
                    k,
                    next_mp
                        .get(&k)
                        .map_or(next_presure, |&p| p.max(next_presure)),
                );
            }

            // go down tunnels
            for &t in &v.tunnels {
                let k = (t, opened);
                next_mp.insert(
                    k,
                    next_mp
                        .get(&k)
                        .map_or(next_presure, |&p| p.max(next_presure)),
                );
            }
        }
        max_preasure = next_max_preasure;
    }
    Ok(max_preasure)
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let solutions = calc_solutions(input, 26)?;

    let groups = solutions
        .iter()
        .map(|(&(_, o), &p)| (o, p))
        .sorted_unstable_by_key(|&(o, _)| o)
        .group_by(|&(o, _)| o);

    let max_by_open = groups
        .into_iter()
        .map(|(o, p)| (o, p.map(|(_, p)| p).max().unwrap_or(0)))
        .filter(|&(o, p)| o != 0 && p != 0)
        .collect_vec();

    let result = max_by_open
        .into_iter()
        .tuple_combinations()
        .filter_map(
            |((o1, p1), (o2, p2))| {
                if o1 & o2 == 0 {
                    Some(p1 + p2)
                } else {
                    None
                }
            },
        )
        .max();

    Ok(result.unwrap_or(0))
}

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

mod parser {
    use super::Valve;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, u32},
        combinator::map,
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        IResult,
    };

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<Valve>> {
        separated_list1(
            line_ending,
            map(
                tuple((
                    preceded(tag("Valve "), alpha1),
                    delimited(
                        tag(" has flow rate="),
                        u32,
                        alt((
                            tag("; tunnels lead to valves "),
                            tag("; tunnel leads to valve "),
                        )),
                    ),
                    separated_list1(tag(", "), alpha1),
                )),
                |(name, flow_rate, tunnels)| Valve {
                    name,
                    flow_rate,
                    tunnels,
                },
            ),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example16.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 1651);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 1707);
        Ok(())
    }
}
