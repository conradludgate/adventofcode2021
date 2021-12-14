#![feature(array_from_fn)]
use std::{array, collections::BTreeMap};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

type Pair = [u8; 2];
type Rules = Vec<(Pair, u8)>;

#[derive(Debug, PartialEq, Clone)]
pub struct Day14<'i> {
    polymer: &'i str,
    rules: Rules,
}

impl<'i> ChallengeParser<'i> for Day14<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let polymer = alpha1;
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

impl<'i> Day14<'i> {
    fn run(self, n: usize) -> usize {
        let mut pairs = BTreeMap::new();
        let bytes = self.polymer.as_bytes();
        for i in 0..(bytes.len() - 1) {
            let pair = [bytes[i], bytes[i + 1]];
            *pairs.entry(pair).or_insert(0_usize) += 1;
        }

        let rules = BTreeMap::from_iter(self.rules);
        for _ in 0..n {
            pairs = step(pairs, &rules);
        }

        let mut counts = BTreeMap::<_, usize>::new();
        pairs
            .into_iter()
            .for_each(|([a, _], c)| *counts.entry(a).or_insert(0) += c);
        let last = bytes.last().unwrap();
        *counts.entry(*last).or_insert(0) += 1;

        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        max - min
    }
}

fn step(polymer: BTreeMap<Pair, usize>, rules: &BTreeMap<Pair, u8>) -> BTreeMap<Pair, usize> {
    let mut new = BTreeMap::new();

    for (pair, count) in polymer {
        match rules.get(&pair) {
            Some(&b) => {
                let [a, c] = pair;
                let pair1 = [a, b];
                let pair2 = [b, c];
                *new.entry(pair1).or_insert(0) += count;
                *new.entry(pair2).or_insert(0) += count;
            }
            None => {
                *new.entry(pair).or_insert(0) += count;
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
