use std::{fmt::Debug, str::FromStr};

use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    error::{FromExternalError, ParseError},
    multi::{many1, separated_list1},
    IResult, Parser,
};

pub fn parse<'a, O, F, E>(f: F) -> impl FnMut(&'a str) -> IResult<&str, O, E>
where
    O: FromStr,
    F: Parser<&'a str, &'a str, E>,
    E: FromExternalError<&'a str, <O as FromStr>::Err>,
{
    map_res(f, O::from_str)
}

pub fn number<O>(input: &str) -> IResult<&str, O>
where
    O: FromStr,
{
    parse(digit1)(input)
}

pub fn lines<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list1(line_ending, f)
}

pub fn all<I, O, E>(result: IResult<I, O, E>) -> O
where
    E: ParseError<I>,
    nom::Err<E>: Debug,
{
    result.unwrap().1
}

pub fn grid<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&str, Vec<Vec<O>>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    lines(many1(f))
}
