use aoc::Challenge;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::line_ending,
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq)]
struct Entry {
    signals: [u8; 10],
    outputs: [u8; 4],
}

#[derive(Debug, PartialEq)]
struct Day08(Vec<Entry>);

fn to_bits(s: &str) -> u8 {
    let mut bits: u8 = 0;
    for b in s.bytes() {
        let b = b - b'a';
        bits |= 1 << b;
    }
    bits
}

impl Challenge for Day08 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        let segments1 = is_a("abcdefg").map(to_bits).separated_array(tag(" "));
        let segments2 = is_a("abcdefg").map(to_bits).separated_array(tag(" "));
        let entry = segments1
            .skip(tag(" | "))
            .and(segments2)
            .map(|(signals, outputs)| Entry { signals, outputs });

        entry.separated_list1(line_ending).map(Day08).parse(input)
    }

    fn part_one(self) -> usize {
        self.0
            .into_iter()
            .flat_map(|entry| {
                entry
                    .outputs
                    .into_iter()
                    .filter(|output| matches!(output.count_ones(), 2 | 4 | 3 | 7))
            })
            // .map(|s| dbg!(s))
            .count()
    }

    fn part_two(self) -> usize {
        self.0.into_iter().map(eval_entry).take(1).sum()
    }
}

//  dddd
// e    a
// e    a
//  ffff
// g    b
// g    b
//  cccc

const A: u8 = 0x01;
const B: u8 = 0x02;
const C: u8 = 0x04;
const D: u8 = 0x08;
const E: u8 = 0x10;
const F: u8 = 0x20;
const G: u8 = 0x40;

const DIGITS: [u8; 10] = [
    D | A | B | C | G | E,
    A | B,
    D | A | F | G | C,
    D | A | F | B | C,
    E | F | A | B,
    D | E | F | B | C,
    D | E | F | G | C | B,
    D | A | B,
    A | B | C | D | E | F | G,
    D | A | B | C | F | E,
];

fn eval_entry(e: Entry) -> usize {
    let mut poss = [0x7f; 7]; // Each output is currently possible for each signal

    e.signals.into_iter().for_each(|s| {
        let d = match s.count_ones() {
            2 => DIGITS[1],
            3 => DIGITS[7],
            4 => DIGITS[4],
            7 => DIGITS[8],
            _ => 0x7f,
        };
        (0..7).for_each(|b| {
            if (s >> b) & 1 == 1 {
                poss[b] &= d
            }
        });
    });

    loop {
        let mut step = false;
        for i in 0..7 {
            let x = poss[i];
            dbg!(x.count_ones());
            if x.count_ones() == 1 {
                for (k, p) in poss.iter_mut().enumerate() {
                    if k != i && *p & x != 0 {
                        step = true;
                        *p ^= x;
                    }
                }
            }
            if x.count_ones() == 2 {
                let j = poss.iter().enumerate().position(|(j, &y)| j != i && y == x).unwrap();
                for (k, p) in poss.iter_mut().enumerate() {
                    if k != i && k != j && *p & x != 0 {
                        step = true;
                        *p ^= x;
                    }
                }
            }
        }
        if !step {
            break;
        }
    }

    println!("{:?}", poss.iter().map(|s| format!("{:02x}", s)).collect::<Vec<_>>());

    e.outputs.into_iter().for_each(|s| {
        let mut signal = 0;
        (0..7).for_each(|b| {
            if (s >> b) & 1 == 1 {
                signal &= poss[b];
            }
        });
        println!("{:02x} {:02x}", s, signal);
    });

    0
}

fn main() {
    Day08::run();
}

#[cfg(test)]
mod tests {
    use super::Day08;
    use aoc::Challenge;

    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn parse() {
        let output = Day08::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day08::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 26);
    }

    #[test]
    fn part_two() {
        let output = Day08::new(INPUT).unwrap().1;
        // assert_eq!(output.part_two(), 61229);
    }

    #[test]
    fn parse_entry() {
        let output =
            Day08::new("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n")
                .unwrap()
                .1;
        // assert_eq!(output.part_two(), 5353);
    }
}
