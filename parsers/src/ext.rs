use std::marker::PhantomData;

use nom::{
    error::{ErrorKind, FromExternalError},
    Err, Parser,
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
}

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

impl<I, O, E, P: Parser<I, O, E>> ParserExt<I, O, E> for P {}
