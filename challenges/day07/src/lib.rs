#![feature(int_abs_diff)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq)]
pub struct Day07(Vec<usize>);

impl<'i> ChallengeParser<'i> for Day07 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        number.separated_list1(tag(",")).map(Self).parse(input)
    }
}

impl Challenge for Day07 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(mut self) -> usize {
        // median
        let index = self.0.len() / 2;
        let pos = *self.0.select_nth_unstable(index).1;
        self.0.into_iter().map(|p| p.abs_diff(pos)).sum()
    }

    fn part_two(self) -> usize {
        // mean
        let s = self.0.iter().sum::<usize>();
        let low = s / self.0.len();
        let high = (s + 1) / self.0.len();

        let f = |pos| self.0.iter().map(|p| p.abs_diff(pos)).map(|n| n * (n + 1) / 2).sum();
        (low..=high).map(f).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Day07;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn parse() {
        let output = Day07::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day07::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 37);
    }

    #[test]
    fn part_two() {
        let output = Day07::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 168);
    }
}
