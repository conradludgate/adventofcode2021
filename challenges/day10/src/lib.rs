use aoc::{Challenge, Parser as ChallengeParser};
use nom::{branch::alt, character::complete::line_ending, IResult, Parser};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Delim {
    Paren, // ()
    Brace, // {}
    Brack, // []
    Angle, // <>
}

#[derive(Debug, PartialEq)]
enum State {
    Open(Delim),
    Close(Delim),
}

fn parse_open(input: &str) -> IResult<&str, Delim> {
    use nom::character::complete::char;
    alt((
        char('(').map(|_| Delim::Paren),
        char('{').map(|_| Delim::Brace),
        char('[').map(|_| Delim::Brack),
        char('<').map(|_| Delim::Angle),
    ))(input)
}

fn parse_close(input: &str) -> IResult<&str, Delim> {
    use nom::character::complete::char;
    alt((
        char(')').map(|_| Delim::Paren),
        char('}').map(|_| Delim::Brace),
        char(']').map(|_| Delim::Brack),
        char('>').map(|_| Delim::Angle),
    ))(input)
}

fn parse_chunk(input: &str) -> IResult<&str, Chunk> {
    alt((parse_open.map(State::Open), parse_close.map(State::Close)))
        .many1()
        .map(Chunk)
        .parse(input)
}

#[derive(Debug)]
pub struct Chunk(Vec<State>);

impl Chunk {
    fn corrupted(self) -> usize {
        let mut stack = vec![];
        for state in self.0 {
            match state {
                State::Open(open) => stack.push(open),
                State::Close(close) => {
                    if stack.pop() != Some(close) {
                        return match close {
                            Delim::Paren => 3,
                            Delim::Brack => 57,
                            Delim::Brace => 1197,
                            Delim::Angle => 25137,
                        };
                    }
                }
            }
        }
        0
    }
    fn incomplete(self) -> usize {
        let mut stack = vec![];
        for state in self.0 {
            match state {
                State::Open(open) => stack.push(open),
                State::Close(close) => {
                    if stack.pop() != Some(close) {
                        return 0;
                    }
                }
            }
        }

        stack.into_iter().rev().fold(0, |acc, x| {
            acc * 5
                + match x {
                    Delim::Paren => 1,
                    Delim::Brack => 2,
                    Delim::Brace => 3,
                    Delim::Angle => 4,
                }
        })
    }
}

#[derive(Debug)]
pub struct Day10(Vec<Chunk>);

impl<'i> ChallengeParser<'i> for Day10 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        parse_chunk.separated_list1(line_ending).map(Self).parse(input)
    }
}

impl Challenge for Day10 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        self.0.into_iter().map(Chunk::corrupted).sum()
    }

    fn part_two(self) -> usize {
        let mut scores = self
            .0
            .into_iter()
            .map(Chunk::incomplete)
            .filter(|&s| s > 0)
            .collect::<Vec<_>>();
        let index = scores.len() / 2;
        *scores.select_nth_unstable(index).1
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_chunk, parse_close, parse_open, Delim};

    use super::Day10;
    use aoc::{Challenge, Parser};

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
    fn open() {
        let (input, output) = parse_open("(EOF").unwrap();
        assert_eq!(output, Delim::Paren);
        assert_eq!(input, "EOF");

        let (input, output) = parse_open("<EOF").unwrap();
        assert_eq!(output, Delim::Angle);
        assert_eq!(input, "EOF");

        let (input, output) = parse_open("[EOF").unwrap();
        assert_eq!(output, Delim::Brack);
        assert_eq!(input, "EOF");

        let (input, output) = parse_open("{EOF").unwrap();
        assert_eq!(output, Delim::Brace);
        assert_eq!(input, "EOF");
    }

    #[test]
    fn close() {
        let (input, output) = parse_close(")EOF").unwrap();
        assert_eq!(output, Delim::Paren);
        assert_eq!(input, "EOF");

        let (input, output) = parse_close(">EOF").unwrap();
        assert_eq!(output, Delim::Angle);
        assert_eq!(input, "EOF");

        let (input, output) = parse_close("]EOF").unwrap();
        assert_eq!(output, Delim::Brack);
        assert_eq!(input, "EOF");

        let (input, output) = parse_close("}EOF").unwrap();
        assert_eq!(output, Delim::Brace);
        assert_eq!(input, "EOF");
    }

    #[test]
    fn chunk() {
        use super::Delim::*;
        use super::State::*;

        let (input, output) = parse_chunk("[({(<(())[]>[[{[]{<()<>>\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(
            output.0,
            vec![
                Open(Brack),
                Open(Paren),
                Open(Brace),
                Open(Paren),
                Open(Angle),
                Open(Paren),
                Open(Paren),
                Close(Paren),
                Close(Paren),
                Open(Brack),
                Close(Brack),
                Close(Angle),
                Open(Brack),
                Open(Brack),
                Open(Brace),
                Open(Brack),
                Close(Brack),
                Open(Brace),
                Open(Angle),
                Open(Paren),
                Close(Paren),
                Open(Angle),
                Close(Angle),
                Close(Angle),
            ]
        );
    }

    #[test]
    fn parse() {
        let output = Day10::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day10::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 26397);
    }

    #[test]
    fn part_two() {
        let output = Day10::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 288957);
    }
}
