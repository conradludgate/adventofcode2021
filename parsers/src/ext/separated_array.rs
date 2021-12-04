use std::marker::PhantomData;

use generic_vec::ArrayVec;
use nom::{error::ParseError, InputLength, Parser};
pub struct SeperatedArray<F, G, O2, const N: usize> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<(O2, [F; N])>,
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
