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
                background: b'.',
            },
        ))
    }
}

impl<'i> Challenge for Day20<'i> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let (new, b, w) = self.apply();

        // for line in new.chunks_exact(w) {
        //     dbg!(unsafe { std::str::from_utf8_unchecked(line) });
        // }

        let new = Day20 {
            rules: self.rules,
            lines: new.chunks_exact(w).collect(),
            background: b,
        };

        let output = new.apply().0;

        output.into_iter().filter(|&c| c == b'#').count()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

impl<'i> Day20<'i> {
    pub fn apply(&self) -> (Vec<u8>, u8, usize) {
        let w = self.lines[0].len() + 4; // padding of 2 on each end
        let h = self.lines.len() + 4; // padding of 2 on each end
        let l = w * h;
        let mut v = Vec::with_capacity(l);

        for i in 0..l {
            v.push(self.rules[self.read(i)]);
        }

        let b = self.rules[if self.background == b'#' { 511 } else { 0 }];

        (v, b, w)
    }

    pub fn read(&self, i: usize) -> usize {
        let l = self.lines[0].len();
        let w = l + 4; // padding of 2 on each end
        let h = self.lines.len() + 4; // padding of 2 on each end
        let (x, y) = (i % w, i / w);

        let mut o: usize = 0;

        // let mut read_line = |line: &[u8]| {
        //     for j in 0..3 {
        //         o <<= 1;
        //         let j = j + x;
        //         let c = if j < 2 || j + 2 >= w {
        //             self.background
        //         } else {
        //             line[j - 2]
        //         };
        //         o |= (c == b'#') as usize;
        //     }
        // };

        for k in 0..3 {
            let k = k + y;
            if k < 2 || k + 2 >= h {
                let c = (self.background == b'#') as usize;
                for _ in 0..3 {
                    o <<= 1;
                    o |= c;
                }
            } else {
                let line = &self.lines[k-2];
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

        // if y == 0 {
        //     read_line(&self.lines[0]);
        // } else if y == 1 {
        //     read_line(&self.lines[0]);
        //     read_line(&self.lines[1]);
        // } else if y == self.lines.len() + 3 {
        //     read_line(&self.lines[y-4]);
        //     let c = (self.background == b'#') as usize;
        //     for i in 0..6 {
        //         o <<= 1;
        //         o |= c;
        //     }
        // } else if y == self.lines.len() + 2 {
        //     read_line(&self.lines[y-4]);
        //     read_line(&self.lines[y-3]);
        //     let c = (self.background == b'#') as usize;
        //     for i in 0..3 {
        //         o <<= 1;
        //         o |= c;
        //     }
        // } else {
        //     let y0 = y - 2;
        //     let y1 = y + 1;
        //     for line in &self.lines[y0..y1] {
        //         read_line(line);
        //     }
        // }

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
        assert_eq!(output.part_two(), 0);
    }
}
