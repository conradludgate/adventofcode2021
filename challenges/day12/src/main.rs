use aoc::Challenge;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq)]
struct Day12(Vec<(String, String)>);

impl Challenge for Day12 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        alpha1
            .skip(tag("-"))
            .and(alpha1)
            .map(|(a, b): (&str, &str)| (a.to_owned(), b.to_owned()))
            .separated_list1(line_ending)
            .map(Day12)
            .parse(input)
    }

    fn part_one(self) -> usize {
        todo!()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}



fn main() {
    Day12::run();
}

#[cfg(test)]
mod tests {
    use super::Day12;
    use aoc::Challenge;

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    #[test]
    fn parse() {
        let output = Day12::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day12::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 10);
    }

    #[test]
    fn part_two() {
        let output = Day12::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
