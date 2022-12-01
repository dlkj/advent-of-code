use std::num::ParseIntError;

use anyhow::Ok;
use itertools::Itertools;

fn main() -> Result<(), anyhow::Error> {
    let input = include_str!("../input.txt");

    println!("{}", solve(input)?);

    Ok(())
}

fn solve(input: &str) -> Result<u32, ParseIntError> {
    let groups = input.lines().group_by(|&l| !l.is_empty());

    groups
        .into_iter()
        .filter_map(|(b, g)| b.then(|| g.map(str::parse::<u32>).sum()))
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().max().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let result = solve(include_str!("../example.txt"));
        assert_eq!(result.unwrap(), 24000);
    }
}
