use nom::{character::complete::digit1, combinator::map_res, error::Error, IResult, Parser};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &str = "\n\n";

pub fn final_parser<'a, R>(
    mut parser: impl Parser<&'a str, R, Error<&'a str>>,
) -> impl FnMut(&'a str) -> Result<R, anyhow::Error> {
    move |input| {
        let r = parser
            .parse(input)
            .map(|(_, r)| r)
            .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
        Ok(r)
    }
}

pub fn dec_int<R>(input: &str) -> IResult<&str, R>
where
    R: std::str::FromStr,
{
    map_res(digit1, str::parse)(input)
}
