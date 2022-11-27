use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day17 {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl<'i> ChallengeParser<'i> for Day17 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let number = map_res(recognize(tuple((opt(tag("-")), digit1))), |x: &str| x.parse::<isize>());
        let range = number.separated_array(tag("..")).map(|[a, b]| a..=b);
        let output = preceded(tag("target area: x="), range.separated_array(tag(", y=")));

        map(output, |[x, y]| Self { x, y }).parse(input)
    }
}

impl Challenge for Day17 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let mut peaks = BTreeMap::new();
        for x in 0..100 {
            for y in 0..2000 {
                let record = self.shoot((x, y));
                if let Some(max) = record.into_iter().map(|(_, y)| y).max() {
                    peaks.insert((x, y), max);
                }
            }
        }
        let (_, y) = peaks.into_iter().max_by_key(|(_, y)| *y).unwrap();

        y as usize
    }

    fn part_two(self) -> usize {
        let mut peaks = BTreeSet::new();
        for x in 0..150 {
            for y in -163..4000 {
                let record = self.shoot((x, y));
                if !record.is_empty() {
                    peaks.insert((x, y));
                }
            }
        }

        peaks.len()
    }
}

impl Day17 {
    fn shoot(&self, mut vel: (isize, isize)) -> Vec<(isize, isize)> {
        let mut pos = (0, 0);
        let mut record = vec![];
        loop {
            pos.0 += vel.0;
            pos.1 += vel.1;
            vel.0 -= vel.0.signum();
            vel.1 -= 1;
            record.push(pos);

            if self.x.contains(&pos.0) && self.y.contains(&pos.1) {
                return record;
            }

            // overshot
            if pos.0 > *self.x.end() || pos.1 < *self.y.start() {
                return vec![];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day17;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn parse() {
        let output = Day17::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day17::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 45);
    }

    #[test]
    fn part_two() {
        let output = Day17::parse(INPUT).unwrap().1;
        output.shoot((6, 0));
        assert_eq!(output.part_two(), 112);
    }
}
