use std::num::ParseIntError;

use itertools::Itertools;

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("../input.txt");

    println!("{}", solve(input)?);

    Ok(())
}

fn solve(input: &str) -> Result<u32, ParseIntError> {
    input
        .lines()
        .group_by(|&l| !l.is_empty())
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
        assert_eq!(result, Ok(24000));
    }
}
