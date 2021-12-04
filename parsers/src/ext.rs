use std::marker::PhantomData;

use generic_vec::ArrayVec;
use nom::{
    error::{ErrorKind, FromExternalError, ParseError},
    Err, InputLength, Parser,
};

pub trait ParserExt<I, O, E>: Parser<I, O, E> {
    fn map_res<G, O2, E2>(self, g: G) -> MapRes<Self, G, O>
    where
        E: FromExternalError<I, E2>,
        G: FnMut(O) -> Result<O2, E2>,
        Self: Sized,
    {
        MapRes {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn separated_list1<G, O2>(self, g: G) -> SeperatedList1<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        SeperatedList1 {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn separated_array<G, O2, const N: usize>(self, g: G) -> SeperatedArray<Self, G, O2, N>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        SeperatedArray {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn skip<G, O2>(self, g: G) -> Skip<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        Skip {
            f: self,
            g,
            _output: PhantomData,
        }
    }
}

impl<I, O, E, P: Parser<I, O, E>> ParserExt<I, O, E> for P {}

pub struct MapRes<F, G, O1> {
    f: F,
    g: G,
    _output: PhantomData<O1>,
}

impl<I, F, G, O1, O2, E, E2> Parser<I, O2, E> for MapRes<F, G, O1>
where
    I: Clone,
    E: FromExternalError<I, E2>,
    G: FnMut(O1) -> Result<O2, E2>,
    F: Parser<I, O1, E>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, O2, E> {
        let i = input.clone();
        let (input, o1) = self.f.parse(input)?;
        match (self.g)(o1) {
            Ok(o2) => Ok((input, o2)),
            Err(e) => Err(Err::Error(E::from_external_error(i, ErrorKind::MapRes, e))),
        }
    }
}

pub struct SeperatedList1<F, G, O2> {
    f: F,
    g: G,
    _output: PhantomData<O2>,
}

impl<I, F, G, O, O2, E> Parser<I, Vec<O>, E> for SeperatedList1<F, G, O2>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, Vec<O>, E> {
        let mut res = Vec::new();

        // Parse the first element
        let (i1, n) = self.f.parse(input)?;
        res.push(n);
        input = i1;

        loop {
            let len = input.input_len();
            match self.g.parse(input.clone()) {
                Err(Err::Error(_)) => return Ok((input, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                    }

                    match self.f.parse(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((input, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            input = i2;
                        }
                    }
                }
            }
        }
    }
}

pub struct SeperatedArray<F, G, O2, const N: usize> {
    f: F,
    g: G,
    _output: PhantomData<(O2, [F; N])>,
}

impl<I, F, G, O, O2, E, const N: usize> Parser<I, [O; N], E> for SeperatedArray<F, G, O2, N>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, [O; N], E> {
        let mut res = ArrayVec::new();

        // Parse the first element
        let (i1, n) = self.f.parse(input)?;
        res.push(n);
        input = i1;

        for _ in 1..N {
            input = self.g.parse(input)?.0;
            let (i1, n) = self.f.parse(input)?;
            res.push(n);
            input = i1
        }

        Ok((input, res.into_array()))
    }
}

pub struct Skip<F, G, O2> {
    f: F,
    g: G,
    _output: PhantomData<O2>,
}

impl<I, F, G, O, O2, E> Parser<I, O, E> for Skip<F, G, O2>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, O, E> {
        let (input, output) = self.f.parse(input)?;
        let (input, _) = self.g.parse(input)?;
        Ok((input, output))
    }
}
