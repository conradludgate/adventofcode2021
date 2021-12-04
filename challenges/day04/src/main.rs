use std::{fmt::Debug, iter::Sum, str::FromStr};

use ansi_term::Style;
use aoc::Challenge;
use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use parsers::{number, parse, separated_list_n, ParserExt};

#[derive(Debug, PartialEq, Clone)]
struct Day04 {
    numbers: Vec<usize>,
    bingos: Vec<Bingo>,
}

#[derive(PartialEq, Clone, Copy)]
struct Bingo(pub [BingoRow; 5]);
#[derive(PartialEq, Clone, Copy)]
struct BingoRow(pub [BingoCell; 5]);
#[derive(PartialEq, Clone, Copy)]
struct BingoCell {
    pub marked: bool,
    pub number: usize,
}

impl Debug for BingoCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style = Style::default();
        style.is_strikethrough = self.marked;
        let n = format!("{:02}", self.number);
        write!(f, "{}", style.paint(n))
    }
}

impl Debug for BingoRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.0 {
            write!(f, "{:?} ", i)?;
        }
        Ok(())
    }
}

impl Debug for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.0 {
            write!(f, "\n{:?}", i)?;
        }
        Ok(())
    }
}

impl<'a> Sum<&'a BingoCell> for usize {
    fn sum<I: Iterator<Item = &'a BingoCell>>(iter: I) -> Self {
        iter.filter(|cell| !cell.marked)
            .map(|cell| cell.number)
            .sum()
    }
}

impl<'a> Sum<&'a BingoRow> for usize {
    fn sum<I: Iterator<Item = &'a BingoRow>>(iter: I) -> Self {
        iter.map(|row| row.0.iter().sum::<usize>()).sum()
    }
}

impl Bingo {
    fn mark(&mut self, n: usize) {
        self.0.iter_mut().for_each(|row| {
            row.0.iter_mut().for_each(|col| {
                col.marked = col.marked || col.number == n;
            })
        })
    }

    fn has_row(&self) -> bool {
        self.0
            .iter()
            .any(|row| row.0.iter().all(|cell| cell.marked))
    }

    fn has_col(&self) -> bool {
        (0..5).any(|col| self.0.iter().all(|row| row.0[col].marked))
    }

    fn count_unmarked(&self) -> usize {
        self.0.iter().sum::<usize>()
    }
}

impl Challenge for Day04 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        let parse_cell = take(2usize)
            .map(str::trim)
            .map_res(FromStr::from_str)
            .map(|n| BingoCell {
                marked: false,
                number: n,
            });

        let parse_board_row = separated_list_n(tag(" "), parse_cell).map(BingoRow);
        let parse_board = separated_list_n(tag("\n"), parse_board_row).map(Bingo);
        let parse_numbers = separated_list1(tag(","), number);

        separated_pair(
            parse_numbers,
            take(2usize),
            separated_list1(tag("\n\n"), parse_board),
        )
        .map(|(numbers, bingos)| Self { numbers, bingos })
        .parse(input)
    }

    fn part_one(&self) -> usize {
        let mut bingos = self.bingos.clone();
        for n in &self.numbers {
            for bingo in &mut bingos {
                bingo.mark(*n);
                if bingo.has_row() || bingo.has_col() {
                    return dbg!(bingo).count_unmarked() * n;
                }
            }
        }
        todo!()
    }

    fn part_two(&self) -> usize {
        let mut bingos = self.bingos.clone();
        for n in &self.numbers {
            let mut i = 0;
            let mut len = bingos.len();
            while i < len {
                let bingo = &mut bingos[i];
                bingo.mark(*n);
                if bingo.has_row() || bingo.has_col() {
                    if len == 1 {
                        return bingo.count_unmarked() * n;
                    }
                    bingos.remove(i);
                    len -= 1;
                } else {
                    i += 1;
                }
            }
        }
        todo!()
    }
}

fn main() {
    Day04::run()
}

#[cfg(test)]
mod tests {
    use aoc::Challenge;

    use crate::Day04;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn parse() {
        let output = Day04::new(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day04::new(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 4512);
    }

    #[test]
    fn part_two() {
        let output = Day04::new(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 1924);
    }
}
