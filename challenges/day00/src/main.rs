use aoc::Challenge;
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq)]
struct Day00(String);

impl Challenge for Day00 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        tag("").map(str::to_owned).map(Day00).parse(input)
    }

    fn part_one(self) -> usize {
        todo!()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

fn main() {
    Day00::run();
}

#[cfg(test)]
mod tests {
    use super::Day00;
    use aoc::Challenge;

    const INPUT: &str = "";

    #[test]
    fn parse() {
        let output = Day00::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day00::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 0);
    }

    #[test]
    fn part_two() {
        let output = Day00::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
