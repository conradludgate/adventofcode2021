use std::marker::PhantomData;

use nom::{error::FromExternalError, IResult, Parser};

pub use self::{
    map_res::MapRes,
    separated_array::SeperatedArray,
    separated_list::SeperatedList1,
    skip::{PrecededBy, Skip},
};

mod map_res;
mod separated_array;
mod separated_list;
mod skip;

pub struct Noop;

impl<I, E> Parser<I, (), E> for Noop {
    fn parse(&mut self, input: I) -> IResult<I, (), E> {
        Ok((input, ()))
    }
}

impl<I, O, E, P: Parser<I, O, E>> ParserExt<I, O, E> for P {}
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

    fn many1<O2>(self) -> SeperatedList1<Self, Noop, O2>
    where
        Self: Sized,
    {
        SeperatedList1 {
            f: self,
            g: Noop,
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

    fn preceded_by<G, O2>(self, g: G) -> PrecededBy<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        PrecededBy {
            f: self,
            g,
            _output: PhantomData,
        }
    }
}
