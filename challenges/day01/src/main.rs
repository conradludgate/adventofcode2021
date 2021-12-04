#![feature(array_windows)]

use aoc::Challenge;
use nom::{character::complete::line_ending, IResult, Parser};
use parsers::{number, ParserExt};

struct Day01(Vec<i32>);

impl Challenge for Day01 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        number.separated_list1(line_ending).map(Day01).parse(input)
    }

    fn part_one(&self) -> usize {
        self.0.array_windows().filter(|[a, b]| b > a).count()
    }

    fn part_two(&self) -> usize {
        let window_sum = self.0.array_windows().map(|[a, b, c]| a + b + c).collect::<Vec<_>>();
        window_sum.array_windows().filter(|[a, b]| b > a).count()
    }
}

fn main() {
    Day01::run();
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
