use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Day00<'i>(&'i str);

impl<'i> ChallengeParser<'i> for Day00<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        tag("").map(Day00).parse(input)
    }
}

impl<'i> Challenge for Day00<'i> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        todo!()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day00;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "";

    #[test]
    fn parse() {
        let output = Day00::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day00::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 0);
    }

    #[test]
    fn part_two() {
        let output = Day00::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
