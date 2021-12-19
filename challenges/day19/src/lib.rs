#![feature(int_abs_diff, array_zip)]

use std::collections::BTreeSet;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use parsers::ParserExt;

type Point = [i32; 3];

fn number(input: &str) -> IResult<&str, i32> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), |x: &str| x.parse::<i32>())(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    number.separated_array(tag(",")).parse(input)
}

#[derive(Debug, PartialEq, Clone)]
struct Scanner(Vec<Point>);

impl Scanner {
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("--- scanner ").and(take_until("\n")).and(line_ending),
            parse_point.separated_list0(line_ending).map(Self),
        )
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day19(Vec<Scanner>);

impl<'i> ChallengeParser<'i> for Day19 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        Scanner::parse.separated_list0(tag("\n\n")).map(Self).parse(input)
    }
}

impl Challenge for Day19 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        self.run().1.len()
    }

    fn part_two(self) -> usize {
        let scanners = self.run().0;
        let mut distances = vec![];
        for (i, a) in scanners.iter().enumerate() {
            for b in &scanners[i..] {
                let dist = a.zip(*b).map(|(a, b)| a.abs_diff(b));
                distances.push(dist[0] + dist[1] + dist[2]);
            }
        }

        distances.into_iter().max().unwrap() as usize
    }
}

impl Day19 {
    fn run(self) -> (Vec<Point>, BTreeSet<Point>) {
        let mut scanners = Vec::<Point>::new();
        let mut beacons = BTreeSet::<Point>::new(); // beacons will need constant look up
        let mut scan_iter = self.0.into_iter();
        beacons.extend(scan_iter.next().unwrap().0);
        scanners.push([0, 0, 0]);

        loop {
            let mut repeat = vec![];

            for mut scanner in scan_iter {
                let mut offset = None;

                let mut i = 0;
                while i < 48 {
                    offset = intersects(&beacons, &scanner.0);
                    if offset.is_some() {
                        break;
                    }

                    // rotate perspective afterwards
                    scanner.0.iter_mut().for_each(|s| rotate(s, i));

                    i += 1;
                }

                if let Some(offset) = offset {
                    beacons.extend(
                        scanner
                            .0
                            .into_iter()
                            .map(|[a, b, c]| [offset[0] + a, offset[1] + b, offset[2] + c]),
                    );
                    scanners.push(offset);
                } else {
                    repeat.push(scanner);
                }
            }

            if repeat.is_empty() {
                break
            }

            scan_iter = repeat.into_iter();
        }

        (scanners, beacons)
    }
}

fn rotate(s: &mut [i32], i: usize) {
    s.rotate_right(1);
    if i % 3 == 0 {
        s.swap(1, 2);
    }
    if i % 6 == 0 {
        s[2] *= -1;
    }
    if i % 12 == 0 {
        s[1] *= -1;
    }
    if i % 24 == 0 {
        s[0] *= -1;
    }
}

fn intersects(beacons: &BTreeSet<Point>, found: &[Point]) -> Option<Point> {
    for b in beacons {
        for f1 in found {
            let offset = [b[0] - f1[0], b[1] - f1[1], b[2] - f1[2]];

            let mut count = 0;

            for f in found {
                let b = [offset[0] + f[0], offset[1] + f[1], offset[2] + f[2]];
                if beacons.contains(&b) {
                    count += 1;
                }
            }

            if count >= 12 {
                return Some(offset);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Day19;
    use aoc::{Challenge, Parser};

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse() {
        let output = Day19::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day19::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 79);
    }

    #[test]
    fn part_two() {
        let output = Day19::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 3621);
    }
}
