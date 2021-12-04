use std::{fmt::Debug, str::FromStr};

use generic_vec::ArrayVec;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    error::{FromExternalError, ParseError},
    multi::{many1, separated_list1},
    IResult, InputLength, Parser,
};

mod ext;
pub use ext::*;

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

pub fn binary(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s| usize::from_str_radix(s, 2))(input)
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

pub fn separated_list_n<I, O, O2, E, F, G, const N: usize>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, [O; N], E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = ArrayVec::new();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        for _ in 1..N {
            i = sep.parse(i)?.0;
            let (i1, n) = f.parse(i)?;
            res.push(n);
            i = i1
        }

        Ok((i, res.into_array()))
    }
}
