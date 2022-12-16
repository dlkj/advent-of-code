use nom::{error::Error, Parser};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

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

pub fn finish_parser_it<'a, O, F>(
    parse_it: nom::combinator::ParserIterator<&'a str, nom::error::Error<&'a str>, F>,
) -> Result<(), anyhow::Error>
where
    F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    let (input, _) = parse_it
        .finish()
        .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;

    if input.trim().is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(format!("unparsed data: {}", input)))
    }
}
