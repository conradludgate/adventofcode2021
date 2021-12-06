use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use aoc::Challenge;
use nom::{bytes::complete::tag, character::complete::line_ending, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Day05(Vec<(Coords, Coords)>);

impl Challenge for Day05 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        let coords = number.skip(tag(",")).and(number).map(|(x, y)| Coords { x, y });
        let line = coords.separated_array(tag(" -> ")).map(|[a, b]| (a, b));
        line.separated_list1(line_ending).map(Self).parse(input)
    }

    fn part_one(self) -> usize {
        count_intersections(self.0.into_iter().filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y))
    }

    fn part_two(self) -> usize {
        count_intersections(self.0.into_iter())
    }
}

fn count_intersections(iter: impl Iterator<Item = (Coords, Coords)>) -> usize {
    let mut sparse = HashMap::new();

    iter.for_each(|(p1, p2)| {
        let mut i = p1;
        let dir = {
            let Coords { x, y } = p2 - p1;
            Coords {
                x: x.clamp(-1, 1),
                y: y.clamp(-1, 1),
            }
        };

        loop {
            *sparse.entry(i).or_insert(0) += 1;
            if i == p2 {
                break;
            }
            i = i + dir;
        }
    });

    sparse.into_iter().filter(|(_, count)| *count > 1).count()
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Coords {
    type Output = Self;

    fn sub(self, r: Self) -> Self {
        Self {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }
}

fn main() {
    Day05::run();
}

#[cfg(test)]
mod tests {
    use aoc::Challenge;

    use crate::Day05;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn parse() {
        let output = Day05::new(INPUT).unwrap().1;
        dbg!(&output);
    }
    #[test]
    fn part_one() {
        let output = Day05::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 5);
    }
    #[test]
    fn part_two() {
        let output = Day05::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 12);
    }
}
