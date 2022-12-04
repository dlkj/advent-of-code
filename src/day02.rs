use itertools::Itertools;

use self::parser::parse;

const INPUT: &str = include_str!("../resources/input02.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Play {
    Rock = 1,     // A
    Paper = 2,    // B
    Scissors = 3, // C
}
impl Play {
    pub fn outcome(self, op: Self) -> Outcome {
        if self == op {
            Outcome::Draw
        } else if self.win() == op {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    pub const fn win(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub const fn lose(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Recommended {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u32, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let answer = input
        .lines()
        .map(parse)
        .map_ok(play_recommended)
        .fold_ok(0, std::ops::Add::add)?;

    Ok(answer)
}

fn part_b(input: &str) -> Result<u32, anyhow::Error> {
    let answer = input
        .lines()
        .map(parse)
        .map_ok(play_for_outcome)
        .fold_ok(0, std::ops::Add::add)?;

    Ok(answer)
}

fn play_recommended((op, rec): (Play, Recommended)) -> u32 {
    let rec_play = match rec {
        Recommended::X => Play::Rock,
        Recommended::Y => Play::Paper,
        Recommended::Z => Play::Scissors,
    };
    rec_play.outcome(op) as u32 + (rec_play as u32)
}

const fn play_for_outcome((op, rec): (Play, Recommended)) -> u32 {
    let (rec_outcome, rec_play) = match rec {
        Recommended::X => (Outcome::Loss, op.win()),
        Recommended::Y => (Outcome::Draw, op),
        Recommended::Z => (Outcome::Win, op.lose()),
    };
    (rec_play as u32) + (rec_outcome as u32)
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::combinator::value;
    use nom::error::Error;
    use nom::sequence::separated_pair;
    use nom::IResult;
    use nom_supreme::final_parser::final_parser;

    use super::Play;
    use super::Recommended;

    pub(super) fn parse(input: &str) -> Result<(Play, Recommended), Error<String>> {
        final_parser(line)(input).map_err(|e: Error<&str>| Error::new(e.input.to_owned(), e.code))
    }

    fn line(input: &str) -> IResult<&str, (Play, Recommended)> {
        separated_pair(play, tag(" "), recommended)(input)
    }

    fn play(input: &str) -> IResult<&str, Play> {
        alt((
            value(Play::Rock, char('A')),
            value(Play::Paper, char('B')),
            value(Play::Scissors, char('C')),
        ))(input)
    }
    fn recommended(input: &str) -> IResult<&str, Recommended> {
        alt((
            value(Recommended::X, char('X')),
            value(Recommended::Y, char('Y')),
            value(Recommended::Z, char('Z')),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example02.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE)?, 15);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE)?, 12);
        Ok(())
    }
}
