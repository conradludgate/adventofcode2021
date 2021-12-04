#![feature(drain_filter)]

use aoc::Challenge;
use nom::{combinator::recognize, IResult, Parser};
use parsers::{binary, lines};

#[derive(Debug, PartialEq)]
struct Day03 {
    pub bit_len: usize,
    pub data: Vec<usize>,
}

impl Challenge for Day03 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        let (_, n) = recognize(binary).parse(input)?;
        lines(binary)
            .map(|data| Self {
                bit_len: n.len(),
                data,
            })
            .parse(input)
    }

    fn part_one(&self) -> usize {
        let n = self.bit_len;

        let counts = self.data.iter().fold(vec![0; n], |mut counts, d| {
            let mut d = *d;
            for i in (0..n).rev() {
                if d & 1 == 1 {
                    counts[i] += 1;
                }
                d >>= 1;
            }
            counts
        });

        let mut gamma = 0;
        let mut epsilon = 0;
        let h = self.data.len() / 2;
        for i in counts {
            gamma <<= 1;
            epsilon <<= 1;

            if i > h {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
        }

        gamma * epsilon
    }

    fn part_two(&self) -> usize {
        let n = self.bit_len;

        let mut oxy = self.data.clone();
        let mut bit = 1 << n;
        while bit > 1 {
            bit >>= 1;

            let oxy_bit = oxy.iter().filter(|&d| *d & bit == bit).count();

            let oxy_keep = if oxy_bit < (oxy.len() - oxy_bit) {
                0
            } else {
                bit
            };

            oxy.drain_filter(|x| *x & bit != oxy_keep).for_each(|_| {});

            if oxy.len() == 1 {
                break;
            }
        }

        let mut co2 = self.data.clone();
        let mut bit = 1 << n;
        while bit > 1 {
            bit >>= 1;

            let co2_bit = co2.iter().filter(|&d| *d & bit == bit).count();

            let co2_keep = if co2_bit >= (co2.len() - co2_bit) {
                0
            } else {
                bit
            };

            co2.drain_filter(|x| *x & bit != co2_keep).for_each(|_| {});

            if co2.len() == 1 {
                break;
            }
        }

        oxy[0] * co2[0]
    }
}

fn main() {
    Day03::run()
}

#[cfg(test)]
mod tests {
    use aoc::Challenge;

    use crate::Day03;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn parse() {
        let output = Day03::new(TEST_INPUT).unwrap().1;

        assert_eq!(
            output,
            Day03 {
                bit_len: 5,
                data: vec![
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
            }
        )
    }

    #[test]
    fn part_one() {
        let output = Day03::new(TEST_INPUT).unwrap().1;
        assert_eq!(output.part_one(), 198)
    }

    #[test]
    fn part_two() {
        let output = Day03::new(TEST_INPUT).unwrap().1;
        assert_eq!(output.part_two(), 230)
    }
}
