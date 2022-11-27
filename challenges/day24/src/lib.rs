use std::time::{Duration, Instant};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space0},
    sequence::tuple,
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Reg {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl Reg {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("x").map(|_| Self::X),
            tag("y").map(|_| Self::Y),
            tag("z").map(|_| Self::Z),
            tag("w").map(|_| Self::W),
        ))(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Value {
    Reg(Reg),
    Number(i64),
}

impl Value {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = space0(input)?;
        alt((Reg::parse.map(Self::Reg), number.map(Self::Number)))(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Inp(Reg),
    Add((Reg, Value)),
    Mul((Reg, Value)),
    Div((Reg, Value)),
    Mod((Reg, Value)),
    Eql((Reg, Value)),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let pair = (Reg::parse, Value::parse);
        alt((
            Reg::parse.preceded_by(tag("inp ")).map(Self::Inp),
            tuple(pair).preceded_by(tag("add ")).map(Self::Add),
            tuple(pair).preceded_by(tag("mul ")).map(Self::Mul),
            tuple(pair).preceded_by(tag("div ")).map(Self::Div),
            tuple(pair).preceded_by(tag("mod ")).map(Self::Mod),
            tuple(pair).preceded_by(tag("eql ")).map(Self::Eql),
        ))(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct State([i64; 4]);

impl State {
    fn get(&self, value: Value) -> i64 {
        match value {
            Value::Reg(reg) => self.get_reg(reg),
            Value::Number(val) => val,
        }
    }
    fn get_reg(&self, reg: Reg) -> i64 {
        self.0[reg as usize]
    }

    fn apply<'i>(&mut self, inst: Instruction, mut input: &'i [i64]) -> &'i [i64] {
        let (reg, v) = match inst {
            Instruction::Inp(reg) => {
                let x;
                (x, input) = input.split_first().unwrap();
                (reg, *x)
            }
            Instruction::Add((reg, x)) => (reg, self.get_reg(reg) + self.get(x)),
            Instruction::Mul((reg, x)) => (reg, self.get_reg(reg) * self.get(x)),
            Instruction::Div((reg, x)) => (reg, self.get_reg(reg) / self.get(x)),
            Instruction::Mod((reg, x)) => (reg, self.get_reg(reg) % self.get(x)),
            // 0 if equal, 1 otherwise
            Instruction::Eql((reg, x)) => (reg, (self.get_reg(reg) != self.get(x)) as i64),
        };
        self.0[reg as usize] = v;

        input
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day24(Vec<Instruction>);

impl<'i> ChallengeParser<'i> for Day24 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        Instruction::parse.separated_list1(line_ending).map(Self).parse(input)
    }
}

impl Day24 {
    fn run(&self, input: &[i64]) -> State {
        let mut state = State::default();
        self.0
            .iter()
            .copied()
            .fold(input, |input, inst| state.apply(inst, input));
        state
    }
}

impl Challenge for Day24 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let mut digits = [9; 14]; //100_000_000_000_000;
        let mut start = Instant::now();
        loop {
            if start.elapsed() > Duration::from_secs(1) {
                println!("{digits:?}");
                start = Instant::now();
            }

            if digits.iter().any(|&x| x == 0) || self.run(&digits).get_reg(Reg::Z) != 0 {
                for digit in digits.iter_mut().rev() {
                    *digit -= 1;
                    if *digit != 0 {
                        break;
                    }
                    *digit = 9;
                }
                continue;
            }

            let mut output = 0;
            for i in digits {
                output *= 10;
                output += i as usize;
            }
            break output;
        }
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::State;

    use super::Day24;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";

    #[test]
    fn parse() {
        let output = Day24::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day24::parse(INPUT).unwrap().1;
        assert_eq!(
            output.run(&[10]),
            // order of bits: 4218
            State([0, 1, 0, 1]),
        );
    }

    #[test]
    fn part_two() {
        let output = Day24::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
