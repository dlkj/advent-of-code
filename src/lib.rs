use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    error::Error,
    sequence::pair,
    IResult, Parser,
};

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
        Err(anyhow::anyhow!("unparsed data"))
    }
}

pub fn dec_iint<R>(input: &str) -> IResult<&str, R>
where
    R: std::str::FromStr,
{
    map_res(recognize(pair(opt(tag("-")), digit1)), str::parse)(input)
}

pub fn dec_uint<R>(input: &str) -> IResult<&str, R>
where
    R: std::str::FromStr,
{
    map_res(digit1, str::parse)(input)
}
