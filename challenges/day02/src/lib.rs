use aoc::{Challenge, Parser as ChallengeParser};
use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use parsers::*;

#[derive(PartialEq, Debug)]
pub enum Dir {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Dir {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            number.preceded_by(tag("forward ")).map(Dir::Forward),
            number.preceded_by(tag("down ")).map(Dir::Down),
            number.preceded_by(tag("up ")).map(Dir::Up),
        ))(input)
    }
}

pub struct Day02(pub Vec<Dir>);

impl<'i> ChallengeParser<'i> for Day02 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        lines(Dir::parse).map(Self).parse(input)
    }
}

impl Challenge for Day02 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let (h, d) = self.0.into_iter().fold((0, 0), |(h, d), x| match x {
            Dir::Forward(x) => (h + x, d),
            Dir::Down(x) => (h, d + x),
            Dir::Up(x) => (h, d - x),
        });

        (h * d) as usize
    }

    fn part_two(self) -> usize {
        let (h, d, _) = self.0.into_iter().fold((0, 0, 0), |(h, d, a), x| match x {
            Dir::Forward(x) => (h + x, d + a * x, a),
            Dir::Down(x) => (h, d, a + x),
            Dir::Up(x) => (h, d, a - x),
        });

        (h * d) as usize
    }
}

#[cfg(test)]
mod tests {
    use aoc::{Challenge, Parser};

    use crate::{Day02, Dir};

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn parse() {
        let output = Day02::parse(INPUT).unwrap().1;

        assert_eq!(
            output.0,
            vec![
                Dir::Forward(5),
                Dir::Down(5),
                Dir::Forward(8),
                Dir::Up(3),
                Dir::Down(8),
                Dir::Forward(2),
            ]
        );
    }

    #[test]
    fn part_one() {
        let output = Day02::parse(INPUT).unwrap().1;

        let x = output.part_one();
        assert_eq!(x, 150);
    }

    #[test]
    fn part_two() {
        let output = Day02::parse(INPUT).unwrap().1;

        let x = output.part_two();
        assert_eq!(x, 900);
    }
}
