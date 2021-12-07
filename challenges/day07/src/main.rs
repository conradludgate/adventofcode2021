#![feature(int_abs_diff)]

use aoc::Challenge;
use nom::{bytes::complete::tag, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq)]
struct Day07(Vec<usize>);

impl Challenge for Day07 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        number.separated_list1(tag(",")).map(Day07).parse(input)
    }

    fn part_one(mut self) -> usize {
        self.0.sort_unstable();
        let low = self.0[0];
        let high = self.0[self.0.len() - 1];

        (low..=high)
            .map(|pos| self.0.iter().map(|p| p.abs_diff(pos)).sum())
            .min_by_key(|f| *f)
            .unwrap()
    }

    fn part_two(mut self) -> usize {
        self.0.sort_unstable();
        let low = self.0[0];
        let high = self.0[self.0.len() - 1];

        (low..=high)
            .map(|pos| self.0.iter().map(|p| p.abs_diff(pos)).map(|n| n * (n + 1) / 2).sum())
            .min_by_key(|f| *f)
            .unwrap()
    }
}

fn main() {
    Day07::run();
}

#[cfg(test)]
mod tests {
    use super::Day07;
    use aoc::Challenge;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn parse() {
        let output = Day07::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day07::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 37);
    }

    #[test]
    fn part_two() {
        let output = Day07::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 168);
    }
}
