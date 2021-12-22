use std::collections::BTreeSet;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{complete::digit1, streaming::line_ending},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        let number = map_res(recognize(tuple((opt(tag("-")), digit1))), |x: &str| x.parse::<i32>());
        number
            .separated_array(tag(".."))
            .map(|[start, end]| Self { start, end })
            .parse(input)
    }

    fn clamp(self, within: Self) -> Self {
        let start = self.start.max(within.start);
        let end = self.end.min(within.end);
        Self { start, end }
    }
}

impl IntoIterator for Range {
    type Item = i32;
    type IntoIter = std::ops::RangeInclusive<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.start ..= self.end
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn parse(input: &str) -> IResult<&str, Self> {
        let assign = Range::parse.preceded_by(take(2usize));
        assign
            .separated_array(tag(","))
            .map(|[x, y, z]| Self { x, y, z })
            .parse(input)
    }

    fn clamp(self, within: Self) -> Self {
        Self {
            x: self.x.clamp(within.x),
            y: self.y.clamp(within.y),
            z: self.z.clamp(within.z),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    On(Cuboid),
    Off(Cuboid),
}

impl State {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            Cuboid::parse.preceded_by(tag("on ")).map(State::On),
            Cuboid::parse.preceded_by(tag("off ")).map(State::Off),
        ))(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day22(Vec<State>);

impl<'i> ChallengeParser<'i> for Day22 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        State::parse.separated_list1(line_ending).map(Self).parse(input)
    }
}

impl Challenge for Day22 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let mut points = BTreeSet::new();
        let within = Range { start: -50, end: 50 };
        let within = Cuboid {
            x: within,
            y: within,
            z: within,
        };

        for state in self.0 {
            match state {
                State::On(c) => {
                    let c = c.clamp(within);
                    for x in c.x {
                        for y in c.y {
                            for z in c.z {
                                points.insert([x, y, z]);
                            }
                        }
                    }
                },
                State::Off(c) => {
                    let c = c.clamp(within);
                    for x in c.x {
                        for y in c.y {
                            for z in c.z {
                                points.remove(&[x, y, z]);
                            }
                        }
                    }
                },
            }
        }

        points.len()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day22;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";

    #[test]
    fn parse() {
        let output = Day22::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day22::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 590784);
    }

    #[test]
    fn part_two() {
        let output = Day22::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
