#![feature(array_from_fn)]
#![feature(array_windows)]
use std::array;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

type Pair = [u8; 2];

#[derive(Debug, PartialEq, Clone)]
pub struct Day14<'i> {
    polymer: &'i [u8],
    rules: Vec<(Pair, u8)>,
}

impl<'i> ChallengeParser<'i> for Day14<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let polymer = alpha1.map(|s: &str| s.as_bytes());
        let pair = take(2_usize).map(|s: &str| array::from_fn(|i| s.as_bytes()[i]));
        let insert = take(1_usize).map(|s: &str| s.as_bytes()[0]);
        let rule = pair.skip(tag(" -> ")).and(insert);
        let rules = rule.separated_list1(line_ending);

        polymer
            .skip(tag("\n\n"))
            .and(rules)
            .map(|(polymer, rules)| Self { polymer, rules })
            .parse(input)
    }
}

impl<'i> Challenge for Day14<'i> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        self.run(10)
    }

    fn part_two(self) -> usize {
        self.run(40)
    }
}

fn ch(a: u8) -> usize {
    (a - b'A') as usize
}

impl<'i> Day14<'i> {
    fn run(self, n: usize) -> usize {
        let mut pairs = [[0_usize; 26]; 26];
        self.polymer.array_windows().for_each(|&[a, b]| {
            pairs[ch(a)][ch(b)] += 1;
        });

        let mut rules = [[26; 26]; 26];
        self.rules
            .into_iter()
            .for_each(|([a, b], c)| rules[ch(a)][ch(b)] = c - b'A');

        for _ in 0..n {
            pairs = step(pairs, &rules);
        }

        let mut counts = [0; 26];
        // only count the second char of each pair to reduce duplicates
        // ensure to count the first character of the polymer string though
        // otherwise it will be lost
        counts[ch(self.polymer[0])] = 1;
        for p in pairs {
            for j in 0..26 {
                counts[j] += p[j];
            }
        }
        let (min, max) = counts.into_iter().fold((usize::MAX, 0), |(min, max), v| {
            (if v < min && v > 0 { v } else { min }, max.max(v))
        });

        max - min
    }
}

fn step(polymer: [[usize; 26]; 26], rules: &[[u8; 26]; 26]) -> [[usize; 26]; 26] {
    let mut new = [[0; 26]; 26];

    for i in 0..26 {
        for j in 0..26 {
            let count = polymer[i][j];
            let k = rules[i][j];
            if k == 26 {
                new[i][j] += count;
            } else {
                let k = k as usize;
                new[i][k] += count;
                new[k][j] += count;
            }
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use super::Day14;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn parse() {
        let output = Day14::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day14::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 1588);
    }

    #[test]
    fn part_two() {
        let output = Day14::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 2188189693529);
    }
}
