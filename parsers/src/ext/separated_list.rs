use std::marker::PhantomData;

use nom::{
    error::{ErrorKind, ParseError},
    Err, InputLength, Parser,
};
pub struct SeperatedList1<F, G, O2> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<O2>,
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