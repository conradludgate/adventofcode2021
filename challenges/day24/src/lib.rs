use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult, Parser,
};
use parsers::ParserExt;
use z3::ast::Ast;
use z3::{ast, Config, Context, Optimize};

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
        let number = map_res(recognize(tuple((opt(tag("-")), digit1))), |x: &str| x.parse::<i64>());
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

#[derive(Debug)]
struct State<'ctx> {
    regs: [ast::BV<'ctx>; 4],
    ctx: &'ctx Context,
    solver: Optimize<'ctx>,
    inputs: Vec<ast::BV<'ctx>>,
    index: usize,
}

impl<'ctx> State<'ctx> {
    fn new(ctx: &'ctx Context) -> Self {
        let solver = Optimize::new(ctx);
        let regs = std::array::from_fn(|_| ast::BV::from_i64(ctx, 0, 64));
        Self {
            regs,
            ctx,
            solver,
            inputs: vec![],
            index: 0,
        }
    }

    fn get(&self, value: Value) -> ast::BV {
        match value {
            Value::Reg(reg) => self.get_reg(reg).clone(),
            Value::Number(val) => ast::BV::from_i64(self.ctx, val, 64),
        }
    }
    fn get_reg(&self, reg: Reg) -> &ast::BV {
        &self.regs[reg as usize]
    }

    fn apply(&mut self, inst: Instruction) {
        let (reg, v) = match inst {
            Instruction::Inp(reg) => {
                let i = ast::BV::new_const(self.ctx, format!("in_{}", self.inputs.len()), 64);
                // 0 < i <= 9 (non zero single digit)
                self.solver.assert(&i.bvsgt(&ast::BV::from_i64(self.ctx, 0, 64)));
                self.solver.assert(&i.bvsle(&ast::BV::from_i64(self.ctx, 9, 64)));
                self.regs[reg as usize] = i.clone();
                self.inputs.push(i);
                return;
            }
            Instruction::Add((reg, x)) => (reg, self.get_reg(reg) + self.get(x)),
            Instruction::Mul((reg, x)) => (reg, self.get_reg(reg) * self.get(x)),
            Instruction::Div((reg, x)) => (reg, self.get_reg(reg).bvsdiv(&self.get(x))),
            Instruction::Mod((reg, x)) => (reg, self.get_reg(reg).bvsmod(&self.get(x))),
            // 0 if equal, 1 otherwise
            Instruction::Eql((reg, x)) => {
                let cond = self.get_reg(reg)._eq(&self.get(x));
                (reg, cond.ite(&self.get(Value::Number(0)), &self.get(Value::Number(1))))
            }
        };
        let output = ast::BV::new_const(self.ctx, format!("out_{}", self.index), 64);
        self.solver.assert(&(output._eq(&v)));
        drop(v);
        self.regs[reg as usize] = output;
        self.index += 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day24(Vec<Instruction>);

impl<'i> ChallengeParser<'i> for Day24 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        Instruction::parse.separated_list1(line_ending).map(Self).parse(input)
    }
}

impl Challenge for Day24 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let mut state = State::new(&ctx);

        self.0.iter().copied().for_each(|inst| state.apply(inst));

        // z == 0
        state
            .solver
            .assert(&(state.get_reg(Reg::Z)._eq(&ast::BV::from_i64(&ctx, 0, 64))));

        // digits into single number
        let input = state
            .inputs
            .into_iter()
            .reduce(|a, b| a * &ast::BV::from_i64(&ctx, 10, 64) + b)
            .unwrap();

        state.solver.maximize(&input);
        state.solver.check(&[]);

        let model = state.solver.get_model().unwrap();
        let res = model.eval(&input, true).unwrap();
        res.as_i64().unwrap() as usize
    }

    fn part_two(self) -> usize {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let mut state = State::new(&ctx);

        self.0.iter().copied().for_each(|inst| state.apply(inst));

        // z == 0
        state
            .solver
            .assert(&(state.get_reg(Reg::Z)._eq(&ast::BV::from_i64(&ctx, 0, 64))));

        // digits into single number
        let input = state.inputs.into_iter().reduce(|a, b| a * 10_i64 + b).unwrap();

        state.solver.minimize(&input);
        state.solver.check(&[]);

        let model = state.solver.get_model().unwrap();
        let res = model.eval(&input, true).unwrap();
        res.as_i64().unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::Day24;
    use aoc::Parser;

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
}
