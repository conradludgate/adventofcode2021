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
        let Self { mut pairs, folds } = self;
        let (axis, index) = folds.into_iter().next().unwrap();

        apply_fold(&mut pairs, axis, index);

        pairs.len()
    }

    fn part_two(self) -> usize {
        let Self { mut pairs, folds } = self;
        folds
            .into_iter()
            .for_each(|(axis, index)| apply_fold(&mut pairs, axis, index));

        // let max_x = pairs.iter().max_by_key(|[x, _]| x).unwrap()[0];
        // let max_y = pairs.iter().max_by_key(|[_, y]| y).unwrap()[1];

        // let width = max_x + 2; // extra space for newline
        // let height = max_y + 1;
        // let mut grid = vec![b' '; width * height];

        // for i in 0..height {
        //     grid[i * width + max_x + 1] = b'\n';
        // }

        // pairs.into_iter().for_each(|[x, y]| {
        //     grid[x + y * width] = b'#';
        // });

        // pairs are sorted by x first, then y
        // so the front of `pairs` should be the dots for the first letter

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

            let index = (x % 5) + y * 5;
            letter |= 1 << index;
        }
        let c = b'A' + LETTERS.iter().position(|&l| l == letter).unwrap() as u8;
        string.push(c);

        println!("{}", String::from_utf8(string).unwrap());

        panic!("");
    }
}

// 5*6 bits. 1 means active.
// Index corresponds to letter in alphabet
#[allow(clippy::unusual_byte_groupings)]
const LETTERS: [u32; 26] = [
    0b_01001_01001_01111_01001_01001_00110, // A
    0b_00111_01001_01001_00111_01001_00111, // B
    0b_00110_01001_00001_00001_01001_00110, // C
    0b_00000_00000_00000_00000_00000_00000, // D
    0b_01111_00001_00001_00111_00001_01111, // E
    0b_00000_00000_00000_00000_00000_00000, // F
    0b_01110_01001_01101_00001_01001_00110, // G
    0b_01001_01001_01001_01111_01001_01001, // H
    0b_00000_00000_00000_00000_00000_00000, // I
    0b_00110_01001_01000_01000_01000_01100, // J
    0b_00000_00000_00000_00000_00000_00000, // K
    0b_00000_00000_00000_00000_00000_00000, // L
    0b_00000_00000_00000_00000_00000_00000, // M
    0b_00000_00000_00000_00000_00000_00000, // N
    0b_00000_00000_00000_00000_00000_00000, // O
    0b_00000_00000_00000_00000_00000_00000, // P
    0b_00000_00000_00000_00000_00000_00000, // Q
    0b_00000_00000_00000_00000_00000_00000, // R
    0b_00000_00000_00000_00000_00000_00000, // S
    0b_00000_00000_00000_00000_00000_00000, // T
    0b_00000_00000_00000_00000_00000_00000, // U
    0b_00000_00000_00000_00000_00000_00000, // V
    0b_00000_00000_00000_00000_00000_00000, // W
    0b_00000_00000_00000_00000_00000_00000, // X
    0b_00000_00000_00000_00000_00000_00000, // Y
    0b_00000_00000_00000_00000_00000_00000, // Z
];

fn apply_fold(points: &mut Vec<[usize; 2]>, axis: char, index: usize) {
    if axis == 'x' {
        points.iter_mut().for_each(|[x, _]| {
            if *x > index {
                *x = 2 * index - *x
            }
        })
    } else if axis == 'y' {
        points.iter_mut().for_each(|[_, y]| {
            if *y > index {
                *y = 2 * index - *y
            }
        })
    }

    points.sort_unstable();
    points.dedup();
}

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

    #[test]
    fn part_two() {
        let output = Day13::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
