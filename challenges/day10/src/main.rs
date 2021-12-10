use aoc::Challenge;
use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{line_ending, one_of},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq)]
enum Delim {
    Paren, // ()
    Brace, // {}
    Brack, // []
    Angle, // <>
}

enum State {
    Open(Delim),
    Close(Delim),
}

fn parse_open(input: &str) -> IResult<&str, Delim> {
    let (input, open) = one_of("({[<")(input)?;
    let open = match open {
        '(' => Delim::Paren,
        '{' => Delim::Brace,
        '[' => Delim::Brack,
        '<' => Delim::Angle,
        t => panic!("? {}", t),
    };

    Ok((input, open))
}

fn parse_close(input: &str) -> IResult<&str, Delim> {
    let (input, close) = one_of(")}]>")(input)?;
    let close = match close {
        ')' => Delim::Paren,
        '}' => Delim::Brace,
        ']' => Delim::Brack,
        '>' => Delim::Angle,
        t => panic!("? {}", t),
    };

    Ok((input, close))
}

fn parse_chunk(mut input: &str) -> IResult<&str, Option<Delim>> {
    let mut stack = vec![];
    loop {
        let (i, state) = alt((
            line_ending.map(|_| None),
            parse_open.map(State::Open).map(Some),
            parse_close.map(State::Close).map(Some),
        ))(input)?;
        match state {
            None => return Ok((input, None)),
            Some(State::Open(open)) => stack.push(open),
            Some(State::Close(close)) => {
                let open = stack.pop().unwrap();
                if open != close {
                    let (input, _) = take_until("\n")(i)?;
                    return Ok((input, Some(close)));
                }
            }
        }
        input = i;
    }
}

#[derive(Debug)]
struct Day10(Vec<Option<Delim>>);

impl Challenge for Day10 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        parse_chunk.separated_list1(line_ending).map(Day10).parse(input)
    }

    fn part_one(self) -> usize {
        self.0
            .into_iter()
            .flatten()
            .map(|x| match x {
                Delim::Paren => 3,
                Delim::Brace => 57,
                Delim::Brack => 1197,
                Delim::Angle => 25137,
            })
            .sum()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

fn main() {
    Day10::run();
}

#[cfg(test)]
mod tests {
    use super::Day10;
    use aoc::Challenge;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn parse() {
        let output = Day10::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day10::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 26397);
    }

    #[test]
    fn part_two() {
        let output = Day10::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
