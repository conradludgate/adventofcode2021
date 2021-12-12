use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug)]
pub struct Day06(Vec<usize>);

impl<'i> ChallengeParser<'i> for Day06 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        number.separated_list1(tag(",")).map(Self).parse(input)
    }
}

impl Challenge for Day06 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        process(80, self.0)
    }

    fn part_two(self) -> usize {
        process(256, self.0)
    }
}

fn process(n: usize, fish: Vec<usize>) -> usize {
    let mut buckets = fish.into_iter().fold([0usize; 9], |mut b, f| {
        b[f] += 1;
        b
    });

    for i in 0..n {
        buckets.swap(i % 7, 7);
        buckets[i % 7] += buckets[7];
        buckets.swap(7, 8);
    }
    buckets.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::Day06;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse() {
        let output = Day06::parse(INPUT).unwrap().1;
        assert_eq!(output.0, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn part_one() {
        let output = Day06::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 5934);
    }

    #[test]
    fn part_two() {
        let output = Day06::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 26984457539);
    }
}
