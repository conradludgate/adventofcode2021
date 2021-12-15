use aoc::{Challenge, Parser as ChallengeParser};
use cached::{cached_key, UnboundCache};
use nom::{
    character::{complete::one_of, streaming::line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day15(Vec<Vec<usize>>);

impl<'i> ChallengeParser<'i> for Day15 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        one_of("0123456789")
            .map(|c| c.to_digit(10).unwrap() as usize)
            .many1()
            .separated_list1(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl<'i> Challenge for Day15 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        dfs_min(&self.0, 0, 0) - self.0[0][0]
    }

    fn part_two(self) -> usize {
        use pathfinding::prelude::dijkstra;
        let n = self.0.len();

        let goal: (usize, usize) = (5 * n - 1, 5 * n - 1);
        let result = dijkstra(
            &(0, 0),
            |&(x, y)| {
                let mut suc = vec![];
                if x > 0 {
                    suc.push((x - 1, y));
                }
                if y > 0 {
                    suc.push((x, y - 1));
                }
                if x + 1 < 5 * n {
                    suc.push((x + 1, y));
                }
                if y + 1 < 5 * n {
                    suc.push((x, y + 1));
                }
                suc.into_iter().map(|(x, y)| ((x, y), s(&self.0, x, y)))
            },
            |&p| p == goal,
        );
        result.unwrap().1
    }
}

// increase v, wrapping around as described
fn s(map: &[Vec<usize>], x: usize, y: usize) -> usize {
    let n = map.len();
    let v = map[y % n][x % n];
    if v == 0 {
        return 0;
    }
    (v + x / n + y / n - 1) % 9 + 1
}

cached_key! {
    DFS_MIN: UnboundCache<(usize, usize), usize> = UnboundCache::new();
    Key = (x, y);
    fn dfs_min(map: &[Vec<usize>], x: usize, y: usize) -> usize = {
        if x >= map[0].len() {
            return usize::MAX;
        }
        if y >= map.len() {
            return usize::MAX;
        }
        if x + 1 == map[0].len() && y + 1 == map.len() {
            return map[y][x];
        }

        let right = dfs_min(map, x + 1, y);
        let down = dfs_min(map, x, y + 1);
        map[y][x] + right.min(down)
    }
}

#[cfg(test)]
mod tests {
    use super::Day15;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn parse() {
        let output = Day15::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day15::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 40);
    }

    #[test]
    fn part_two() {
        let output = Day15::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 315);
    }
}
