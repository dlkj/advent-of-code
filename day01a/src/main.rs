use std::num::ParseIntError;

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n\n";

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("../input.txt");

    println!("{}", solve(input)?);

    Ok(())
}

fn solve(input: &str) -> Result<u32, ParseIntError> {
    let answer = input
        .split(DOUBLE_LINE_ENDING)
        .map(|s| s.lines().map(str::parse::<u32>).sum())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .unwrap_or(0);

    Ok(answer)
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
