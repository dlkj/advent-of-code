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
        .collect::<Result<Vec<u32>, _>>()
        .map(|v| {
            v.into_iter()
                .sorted_unstable_by(|a, b| b.cmp(a))
                .take(3)
                .sum()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let result = solve(include_str!("../example.txt"));
        assert_eq!(result, Ok(45000));
    }
}
