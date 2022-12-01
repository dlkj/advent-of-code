use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("../input.txt");

    println!("{}", solve(input)?);

    Ok(())
}

fn solve(input: &str) -> Result<u32, ParseIntError> {
    input.parse()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn example() {
        let result = solve(include_str!("../example.txt"));
        assert_eq!(result.is_err(), true);
    }
}
