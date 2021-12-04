#![feature(drain_filter)]
use std::{fmt::Debug, str::FromStr};

use ansi_term::Style;
use aoc::Challenge;
use nom::{
    bytes::complete::{tag, take},
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone)]
struct Day04 {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(PartialEq, Clone, Copy)]
struct Board(pub [Row; 5]);
#[derive(PartialEq, Clone, Copy)]
struct Row(pub [Cell; 5]);
#[derive(PartialEq, Clone, Copy)]
struct Cell {
    pub marked: bool,
    pub number: usize,
}

impl Board {
    fn is_bingo(&mut self, n: usize) -> bool {
        self.0.iter_mut().for_each(|row| {
            row.0.iter_mut().for_each(|col| {
                col.marked = col.marked || col.number == n;
            })
        });
        self.has_row() || self.has_col()
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
        self.0
            .iter()
            .map(|row| {
                row.0
                    .iter()
                    .filter(|cell| !cell.marked)
                    .map(|cell| cell.number)
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Challenge for Day04 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: &str) -> IResult<&str, Self> {
        let parse_cell = take(2usize) // each bingo cell is 2 chars
            .map(str::trim) // remove leading space on single digit nums
            .map_res(FromStr::from_str) // convert digits to decimal
            .map(|n| Cell {
                marked: false,
                number: n,
            });

        let parse_numbers = number.separated_list1(tag(",")); // bingo numbers are seperated by commas
        let parse_boards = parse_cell
            .separated_array(tag(" ")) // cells are seperated by spaces
            .map(Row) // 5 cells form a row
            .separated_array(tag("\n")) // rows are seperated by newlines
            .map(Board) // 5 rows form a board
            .separated_list1(tag("\n\n")); // boards are seperated by double newlines

        parse_numbers // the input consists of the bingo numbers
            .skip(tag("\n\n")) // then 2 new lines
            .and(parse_boards) // then the bingo boards
            .map(|(numbers, boards)| Self { numbers, boards })
            .parse(input)
    }

    fn part_one(&self) -> usize {
        let mut boards = self.boards.clone();
        self.numbers
            .iter()
            .find_map(|n| {
                boards
                    .iter_mut()
                    .filter_map(|b| b.is_bingo(*n).then(|| b.count_unmarked() * n))
                    .next()
            })
            .unwrap()
    }

    fn part_two(&self) -> usize {
        let mut boards = self.boards.clone();
        self.numbers
            .iter()
            .find_map(|n| {
                let completed = boards.drain_filter(|board| board.is_bingo(*n)).last();
                boards
                    .is_empty()
                    .then(|| completed)
                    .flatten()
                    .map(|board| board.count_unmarked() * n)
            })
            .unwrap()
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

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style = Style::default();
        style.is_strikethrough = self.marked;
        let n = format!("{:02}", self.number);
        write!(f, "{}", style.paint(n))
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|cell| write!(f, "{:?} ", cell))
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|row| write!(f, "\n{:?}", row))
    }
}
