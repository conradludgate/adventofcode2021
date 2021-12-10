use aoc::Challenge;
use nom::{branch::alt, bytes::complete::tag, character::complete::line_ending, IResult, Parser};
use parsers::ParserExt;

#[derive(Debug, PartialEq)]
enum Delim {
    Paren, // ()
    Brace, // {}
    Brack, // []
    Angle, // <>
}

#[derive(Debug)]
enum State {
    Open(Delim),
    Close(Delim),
}

fn parse_open(input: &str) -> IResult<&str, Delim> {
    alt((
        tag("(").map(|_| Delim::Paren),
        tag("{").map(|_| Delim::Brace),
        tag("[").map(|_| Delim::Brack),
        tag("<").map(|_| Delim::Angle),
    ))(input)
}

fn parse_close(input: &str) -> IResult<&str, Delim> {
    alt((
        tag(")").map(|_| Delim::Paren),
        tag("}").map(|_| Delim::Brace),
        tag("]").map(|_| Delim::Brack),
        tag(">").map(|_| Delim::Angle),
    ))(input)
}

fn parse_chunk(input: &str) -> IResult<&str, Chunk> {
    alt((parse_open.map(State::Open), parse_close.map(State::Close)))
        .many1()
        .map(Chunk)
        .parse(input)
}

#[derive(Debug)]
struct Chunk(Vec<State>);

impl Chunk {
    fn corrupted(self) -> usize {
        let mut stack = vec![];
        for state in self.0 {
            match state {
                State::Open(open) => stack.push(open),
                State::Close(close) => {
                    let open = stack.pop().unwrap();
                    if open != close {
                        return match close {
                            Delim::Paren => 3,
                            Delim::Brace => 57,
                            Delim::Brack => 1197,
                            Delim::Angle => 25137,
                        };
                    }
                }
            }
        }
        0
    }
}

#[derive(Debug)]
struct Day10(Vec<Chunk>);

impl Challenge for Day10 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        parse_chunk.separated_list1(line_ending).map(Day10).parse(input)
    }

    fn part_one(self) -> usize {
        self.0.into_iter().map(Chunk::corrupted).sum()
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
