use aoc::Challenge;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq)]
struct Day12(Vec<(String, String)>);

impl Challenge for Day12 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        alpha1
            .skip(tag("-"))
            .and(alpha1)
            .map(|(a, b): (&str, &str)| (a.to_owned(), b.to_owned()))
            .separated_list1(line_ending)
            .map(Day12)
            .parse(input)
    }

    fn part_one(self) -> usize {
        dfs(&self.0, vec![], false)
    }

    fn part_two(self) -> usize {
        dfs(&self.0, vec![], true)
    }
}

fn dfs<'a>(map: &'a [(String, String)], path: Vec<&'a str>, part2: bool) -> usize {
    let last = path.last().map_or("start", |&x| x);
    if last == "end" {
        return 1;
    }

    map.iter()
        .map(|(a, b)| {
            let to = if a == last {
                b
            } else if b == last {
                a
            } else {
                return 0;
            };

            // cannot revisit start
            if to == "start" {
                return 0;
            }

            let mut p2 = part2;

            // is ascii lowercase.
            // If lowercase node was already in our path, skip
            // If part2 still applies, we can revisit a single small cave only once
            if to.as_bytes()[0] >= b'a' && path.contains(&to.as_str()) && !std::mem::take(&mut p2) {
                return 0;
            }

            let mut new = path.clone();
            new.push(to);
            dfs(map, new, p2)
        })
        .sum()
}

fn main() {
    Day12::run();
}

#[cfg(test)]
mod tests {
    use super::Day12;
    use aoc::Challenge;

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    #[test]
    fn parse() {
        let output = Day12::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day12::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 10);
    }

    #[test]
    fn part_two() {
        let output = Day12::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 36);
    }
}
