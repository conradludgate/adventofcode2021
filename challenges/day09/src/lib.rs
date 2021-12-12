use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    character::complete::{line_ending, one_of},
    IResult, Parser,
};
use parsers::ParserExt;
#[derive(Debug, PartialEq)]
pub struct Day09(Vec<Vec<u32>>);

impl<'i> ChallengeParser<'i> for Day09 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        one_of("0123456789")
            .map_res(|c| c.to_digit(10).ok_or(()))
            .many1()
            .separated_list0(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Day09 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let mut risk = 0;
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let mut to_check = vec![];
                if y > 0 {
                    to_check.push((x, y - 1));
                }
                if y < self.0.len() - 1 {
                    to_check.push((x, y + 1));
                }
                if x > 0 {
                    to_check.push((x - 1, y));
                }
                if x < row.len() - 1 {
                    to_check.push((x + 1, y));
                }

                if to_check.into_iter().all(|(x, y)| self.0[y][x] > *cell) {
                    risk += *cell + 1;
                }
            }
        }

        risk as usize
    }

    fn part_two(mut self) -> usize {
        let mut sinks = vec![];
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let mut to_check = vec![];
                if y > 0 {
                    to_check.push((x, y - 1));
                }
                if y < self.0.len() - 1 {
                    to_check.push((x, y + 1));
                }
                if x > 0 {
                    to_check.push((x - 1, y));
                }
                if x < row.len() - 1 {
                    to_check.push((x + 1, y));
                }

                if to_check.into_iter().all(|(x1, y1)| self.0[y1][x1] > *cell) {
                    sinks.push((x, y));
                }
            }
        }

        let mut basins = sinks
            .into_iter()
            .map(|(x, y)| count(&mut self.0, (x as isize, y as isize)))
            .collect::<Vec<_>>();

        let n = basins.len();
        let (_, n3, n21) = basins.select_nth_unstable(n - 3);
        *n3 * n21[0] * n21[1]
    }
}

fn count(cells: &mut [Vec<u32>], pos: (isize, isize)) -> usize {
    let (x, y) = pos;
    if x < 0 || y < 0 || y >= cells.len() as isize || x >= cells[0].len() as isize {
        return 0;
    }

    if cells[y as usize][x as usize] >= 9 {
        return 0;
    }

    // mark cell
    cells[y as usize][x as usize] = 10;

    1 + count(cells, (x + 1, y)) + count(cells, (x, y + 1)) + count(cells, (x - 1, y)) + count(cells, (x, y - 1))
}

#[cfg(test)]
mod tests {
    use super::Day09;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn parse() {
        let output = Day09::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day09::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 15);
    }

    #[test]
    fn part_two() {
        let output = Day09::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 1134);
    }
}
