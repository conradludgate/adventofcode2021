use aoc::Challenge;
use nom::{IResult, Parser, bytes::complete::tag};

struct Day00(String);

impl Challenge for Day00 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        tag("").map(str::to_owned).map(Day00).parse(input)
    }

    fn part_one(&self) -> usize {
        todo!()
    }

    fn part_two(&self) -> usize {
        todo!()
    }
}

fn main() {
    Day00::run()
}
