use aoc::Challenge;
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult, Parser};
use parsers::{lines, number};

#[derive(PartialEq, Debug)]
enum Dir {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Dir {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tuple((tag("forward "), number)), |(_, n)| Dir::Forward(n)),
            map(tuple((tag("down "), number)), |(_, n)| Dir::Down(n)),
            map(tuple((tag("up "), number)), |(_, n)| Dir::Up(n)),
        ))(input)
    }
}

struct Day02(pub Vec<Dir>);

impl Challenge for Day02 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        lines(Dir::parse).map(Day02).parse(input)
    }

    fn part_one(&self) -> usize {
        let (x, y) = self.0.iter().fold((0, 0), |(x, y), d| match d {
            Dir::Forward(d) => (x + d, y),
            Dir::Down(d) => (x, y + d),
            Dir::Up(d) => (x, y - d),
        });

        (x * y) as usize
    }

    fn part_two(&self) -> usize {
        let (x, y, _) = self.0.iter().fold((0, 0, 0), |(x, y, a), d| match d {
            Dir::Forward(d) => (x + d, y + a * d, a),
            Dir::Down(d) => (x, y, a + d),
            Dir::Up(d) => (x, y, a - d),
        });

        (x * y) as usize
    }
}

fn main() {
    Day02::run()
}

#[cfg(test)]
mod tests {
    use aoc::Challenge;

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
        let output = Day02::new(INPUT).unwrap().1;

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
        let output = Day02::new(INPUT).unwrap().1;

        let x = output.part_one();
        assert_eq!(x, 150);
    }

    #[test]
    fn part_two() {
        let output = Day02::new(INPUT).unwrap().1;

        let x = output.part_two();
        assert_eq!(x, 900);
    }
}
