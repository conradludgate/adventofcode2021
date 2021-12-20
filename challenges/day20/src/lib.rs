use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::line_ending,
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day20<'i> {
    // ascii strings :)
    rules: &'i [u8],
    lines: Vec<&'i [u8]>,
    background: u8,
}

impl<'i> ChallengeParser<'i> for Day20<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let (input, rules) = is_a(".#")(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, lines) = is_a(".#")
            .map(|s: &str| s.as_bytes())
            .separated_list1(line_ending)
            .parse(input)?;

        Ok((
            input,
            Self {
                rules: rules.as_bytes(),
                lines,
                background: b'.', // background starts off empty
            },
        ))
    }
}

impl<'i> Challenge for Day20<'i> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        // part 1 requires 2 enchances
        self.enhance(2 - 1)
    }

    fn part_two(self) -> usize {
        // part 2 requires 50 enhances
        self.enhance(50 - 1)
    }
}

impl<'i> Day20<'i> {
    /// repeatedly enchance the image i+1 times
    /// returning the number of activated cells at the end
    pub fn enhance(&self, i: usize) -> usize {
        let (buf, b, w) = self.apply();

        if i == 0 {
            buf.into_iter().filter(|&c| c == b'#').count()
        } else {
            let new = Day20 {
                rules: self.rules,
                lines: buf.chunks_exact(w).collect(),
                background: b,
            };

            // recursive because __lifetimes__
            new.enhance(i - 1)
        }
    }

    /// enhance an image once into a single flat buffer
    pub fn apply(&self) -> (Vec<u8>, u8, usize) {
        let w = self.lines[0].len() + 4; // padding of 2 on each end
        let h = self.lines.len() + 4; // padding of 2 on each end
        let l = w * h;
        let mut v = Vec::with_capacity(l);

        for i in 0..l {
            v.push(self.rules[self.read(i)]);
        }

        // make sure to account for the infinite image background
        let b = self.rules[if self.background == b'#' { 511 } else { 0 }];

        (v, b, w)
    }

    /// read 3x3 (9bit) binary value from image grid
    /// including padding
    pub fn read(&self, i: usize) -> usize {
        let l = self.lines[0].len();
        let w = l + 4; // padding of 2 on each end
        let h = self.lines.len() + 4; // padding of 2 on each end
        let (x, y) = (i % w, i / w);

        let mut o: usize = 0;

        for k in 0..3 {
            let k = k + y;
            if k < 2 || k + 2 >= h {
                let c = (self.background == b'#') as usize;
                for _ in 0..3 {
                    o <<= 1;
                    o |= c;
                }
            } else {
                let line = &self.lines[k - 2];
                for j in 0..3 {
                    o <<= 1;
                    let j = j + x;
                    let c = if j < 2 || j + 2 >= w {
                        self.background
                    } else {
                        line[j - 2]
                    };
                    o |= (c == b'#') as usize;
                }
            }
        }

        o
    }
}

#[cfg(test)]
mod tests {
    use super::Day20;
    use aoc::{Challenge, Parser};

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse() {
        let output = Day20::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day20::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 35);
    }

    #[test]
    fn part_two() {
        let output = Day20::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 3351);
    }
}
