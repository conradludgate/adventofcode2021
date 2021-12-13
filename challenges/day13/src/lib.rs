use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone)]
pub struct Day13 {
    pairs: Vec<[usize; 2]>,
    folds: Vec<(char, usize)>,
}

impl<'i> ChallengeParser<'i> for Day13 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let pair = number.separated_array(tag(","));
        let pairs = pair.separated_list1(line_ending);
        let fold = one_of("xy").preceded_by(tag("fold along ")).skip(tag("=")).and(number);
        let folds = fold.separated_list1(line_ending);

        pairs
            .skip(tag("\n\n"))
            .and(folds)
            .map(|(pairs, folds)| Self { pairs, folds })
            .parse(input)
    }
}

impl<'i> Challenge for Day13 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let Self { pairs, folds } = self;

        let fold = folds.into_iter().next().unwrap();
        apply_fold(pairs, fold).len()
    }

    fn part_two(self) -> usize {
        let code = self.part2();
        // my auto submitter doesn't support strings just yet
        // so I'm just gonna panic the output to submit manually
        panic!("{}", code);
    }
}

fn apply_fold(mut points: Vec<[usize; 2]>, (axis, index): (char, usize)) -> Vec<[usize; 2]> {
    let i = ((axis as u8) - b'x') as usize;
    points.iter_mut().for_each(|p| {
        if p[i] > index {
            p[i] = 2 * index - p[i]
        }
    });

    points.sort_unstable();
    points.dedup();
    points
}

impl Day13 {
    pub fn part2(self) -> String {
        let Self { pairs, folds } = self;
        let pairs = folds
            .into_iter()
            .fold(pairs, apply_fold);

        // 'OCR'
        // Each letter fits in a 4 * 6 dot grid (with a space between to make 5 * 6)
        // 4*6 = 24 which fits in a u32. set all the bits on for that number and you
        // can get it's corresponding letter from the hardcoded `LETTERS` list

        // `pairs` are also sorted first by x, which means that all the first set of pairs will only be
        // for the first letter, then the second, etc

        let mut string = vec![];
        let mut letter = 0;
        let mut offset = 0;
        for [x, y] in pairs {
            if x / 5 > offset {
                let c = b'A' + LETTERS.iter().position(|&l| l == letter).unwrap() as u8;
                string.push(c);
                letter = 0;
                offset = x / 5;
            }

            let index = (x % 5) + y * 4;
            letter |= 1 << index;
        }
        let c = b'A' + LETTERS.iter().position(|&l| l == letter).unwrap() as u8;
        string.push(c);

        String::from_utf8(string).unwrap()
    }
}

// 5*6 bits. 1 means active.
// Index corresponds to letter in alphabet
#[allow(clippy::unusual_byte_groupings)]
const LETTERS: [u32; 26] = [
    0b_1001_1001_1111_1001_1001_0110, // A
    0b_0111_1001_1001_0111_1001_0111, // B
    0b_0110_1001_0001_0001_1001_0110, // C
    0b_0000_0000_0000_0000_0000_0000, // D
    0b_1111_0001_0001_0111_0001_1111, // E
    0b_0000_0000_0000_0000_0000_0000, // F
    0b_1110_1001_1101_0001_1001_0110, // G
    0b_1001_1001_1001_1111_1001_1001, // H
    0b_0000_0000_0000_0000_0000_0000, // I
    0b_0110_1001_1000_1000_1000_1100, // J
    0b_0000_0000_0000_0000_0000_0000, // K
    0b_0000_0000_0000_0000_0000_0000, // L
    0b_0000_0000_0000_0000_0000_0000, // M
    0b_0000_0000_0000_0000_0000_0000, // N
    0b_0000_0000_0000_0000_0000_0000, // O
    0b_0000_0000_0000_0000_0000_0000, // P
    0b_0000_0000_0000_0000_0000_0000, // Q
    0b_0000_0000_0000_0000_0000_0000, // R
    0b_0000_0000_0000_0000_0000_0000, // S
    0b_0000_0000_0000_0000_0000_0000, // T
    0b_0000_0000_0000_0000_0000_0000, // U
    0b_0000_0000_0000_0000_0000_0000, // V
    0b_0000_0000_0000_0000_0000_0000, // W
    0b_0000_0000_0000_0000_0000_0000, // X
    0b_0000_0000_0000_0000_0000_0000, // Y
    0b_0000_0000_0000_0000_0000_0000, // Z
];

#[cfg(test)]
mod tests {
    use super::Day13;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn parse() {
        let output = Day13::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day13::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 17);
    }
}
