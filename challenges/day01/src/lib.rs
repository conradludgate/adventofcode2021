#![feature(array_windows)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{character::complete::line_ending, IResult, Parser};
use parsers::{number, ParserExt};

pub struct Day01(Vec<i32>);

impl<'i> ChallengeParser<'i> for Day01 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        number.separated_list1(line_ending).map(Self).parse(input)
    }
}

impl Challenge for Day01 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        self.0.array_windows().filter(|[a, b]| b > a).count()
    }

    fn part_two(self) -> usize {
        self.0.array_windows().filter(|[a, _, _, d]| d > a).count()
    }
}

#[cfg(test)]
mod tests {
    use aoc::Challenge;

    use crate::Day01;

    #[test]
    fn part_one() {
        let challenge = Day01(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(challenge.part_one(), 7)
    }

    #[test]
    fn part_two() {
        let challenge = Day01(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(challenge.part_two(), 5)
    }
}
