#![feature(generic_associated_types)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day12<'a>(Vec<(&'a str, &'a str)>);

impl<'i> ChallengeParser<'i> for Day12<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        alpha1
            .skip(tag("-"))
            .and(alpha1)
            // .map(|(a, b): (&str, &str)| (a.to_owned(), b.to_owned()))
            .separated_list1(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Day12<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        self.dfs(vec![], false)
    }

    fn part_two(self) -> usize {
        self.dfs(vec![], true)
    }
}

impl<'a> Day12<'a> {
    fn dfs(&'a self, path: Vec<&'a str>, revisit: bool) -> usize {
        let last = path.last().map_or("start", |&x| x);
        // found the end, so we've got a valid path
        if last == "end" {
            return 1;
        }

        self.0
            .iter()
            .copied()
            .map(|(a, b)| {
                let to = if a == last {
                    b
                } else if b == last {
                    a
                } else {
                    return 0;
                };

                // cannot revisit start
                if to == "start" {
                    return 0;
                }

                let mut revisit = revisit;

                // is ascii lowercase.
                // If lowercase node was already in our path, skip
                // If part2 still applies, we can revisit a single small cave only once
                if to.as_bytes()[0] >= b'a' && path.contains(&to) && !std::mem::take(&mut revisit) {
                    return 0;
                }

                let mut new = path.clone();
                new.push(to);
                self.dfs(new, revisit)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day12;
    use aoc::{Challenge, Parser};

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
        let output = Day12::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day12::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 10);
    }

    #[test]
    fn part_two() {
        let output = Day12::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 36);
    }
}
