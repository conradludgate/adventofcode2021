#![feature(int_abs_diff)]
use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    character::{complete::one_of, streaming::line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day15(Vec<Vec<usize>>);

impl<'i> ChallengeParser<'i> for Day15 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        one_of("0123456789")
            .map(|c| c.to_digit(10).unwrap() as usize)
            .many1()
            .separated_list1(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl<'i> Challenge for Day15 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let n = self.0.len();
        let goal = (n - 1, n - 1);
        self.minpath(goal)
    }

    fn part_two(self) -> usize {
        let n = self.0.len();
        let goal = (5 * n - 1, 5 * n - 1);
        self.minpath(goal)
    }
}

impl Day15 {
    fn s(&self, x: usize, y: usize) -> usize {
        let n = self.0.len();
        let v = self.0[y % n][x % n];
        (v + x / n + y / n - 1) % 9 + 1
    }

    fn minpath(&self, end: (usize, usize)) -> usize {
        use pathfinding::prelude::astar;
        let result = astar(
            &(0, 0),
            |&(x, y)| {
                let mut suc = vec![];
                if x > 0 {
                    suc.push((x - 1, y));
                }
                if y > 0 {
                    suc.push((x, y - 1));
                }
                if x < end.0 {
                    suc.push((x + 1, y));
                }
                if y < end.1 {
                    suc.push((x, y + 1));
                }
                suc.into_iter().map(|(x, y)| ((x, y), self.s(x, y)))
            },
            |&(x, y)| x.abs_diff(end.0) + y.abs_diff(end.1),
            |&p| p == end,
        );
        result.unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::Day15;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn parse() {
        let output = Day15::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day15::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 40);
    }

    #[test]
    fn part_two() {
        let output = Day15::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 315);
    }
}
