use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::one_of, streaming::line_ending},
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(PartialEq, Clone)]
enum Tree {
    Pair(Box<Tree>, Box<Tree>),
    Value(usize),
}

impl Tree {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                delimited(tag("["), separated_pair(Tree::parse, tag(","), Tree::parse), tag("]")),
                |(x, y)| Tree::Pair(Box::new(x), Box::new(y)),
            ),
            map(one_of("0123456789"), |c| Tree::Value(c.to_digit(10).unwrap() as usize)),
        ))(input)
    }

    fn eval(self) -> usize {
        match self {
            Tree::Pair(x, y) => 3 * x.eval() + 2 * y.eval(),
            Tree::Value(v) => v as usize,
        }
    }

    fn left(&mut self) -> &mut usize {
        match self {
            Tree::Pair(x, _) => x.left(),
            Tree::Value(v) => v,
        }
    }

    fn right(&mut self) -> &mut usize {
        match self {
            Tree::Pair(_, y) => y.right(),
            Tree::Value(v) => v,
        }
    }

    fn add_to(self, rhs: Self) -> Self {
        let mut t = Tree::Pair(Box::new(self), Box::new(rhs));
        t.reduce();
        t
    }

    fn reduce(&mut self) {
        // explode first, then split afterwards.
        while self.explode_impl(0, None, None) || self.split_impl() {}
    }

    fn explode_impl(&mut self, depth: usize, left: Option<&mut usize>, right: Option<&mut usize>) -> bool {
        match self {
            Tree::Pair(x, y) => {
                if depth >= 4 {
                    if let (Tree::Value(x), Tree::Value(y)) = (&mut **x, &mut **y) {
                        if let Some(l) = left {
                            *l += *x
                        }
                        if let Some(r) = right {
                            *r += *y
                        }
                        *self = Tree::Value(0);
                        return true;
                    }
                }

                x.explode_impl(depth + 1, left, Some(y.left())) || y.explode_impl(depth + 1, Some(x.right()), right)
            }
            Tree::Value(_) => false,
        }
    }

    fn split_impl(&mut self) -> bool {
        match self {
            Tree::Pair(x, y) => x.split_impl() || y.split_impl(),
            Tree::Value(v) => {
                if *v >= 10 {
                    let x = *v / 2;
                    let y = (*v + 1) / 2;
                    *self = Tree::Pair(Box::new(Tree::Value(x)), Box::new(Tree::Value(y)));
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day18(Vec<Tree>);

impl<'i> ChallengeParser<'i> for Day18 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        Tree::parse.separated_list0(line_ending).map(Self).parse(input)
    }
}

impl Challenge for Day18 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        // sum them all in order
        let tree = self.0.into_iter().reduce(|a, b| a.add_to(b)).unwrap();
        tree.eval()
    }

    fn part_two(self) -> usize {
        let mut max = 0;
        for i in 0..(self.0.len() - 1) {
            let x = &self.0[i];
            for j in i..self.0.len() {
                let y = &self.0[j];
                max = max.max(x.clone().add_to(y.clone()).eval());
                max = max.max(y.clone().add_to(x.clone()).eval());
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Day18;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn parse() {
        let output = Day18::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day18::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 4140);
    }

    #[test]
    fn part_two() {
        let output = Day18::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 3993);
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(x, y) => write!(f, "[{:?},{:?}]", x, y),
            Self::Value(v) => write!(f, "{:?}", v),
        }
    }
}
