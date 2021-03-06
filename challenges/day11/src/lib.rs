use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    character::complete::{line_ending, one_of},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day11([[u8; 10]; 10]);

impl<'i> ChallengeParser<'i> for Day11 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        one_of("0123456789")
            .map(|c| (c as u8) - b'0')
            .array()
            .separated_array(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Day11 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let mut grid = self.0;
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += flash_step(&mut grid);
        }
        flashes
    }

    fn part_two(self) -> usize {
        let mut grid = self.0;
        let mut i = 1;
        loop {
            // all 10x10 octopus flash
            if flash_step(&mut grid) == 100 {
                return i;
            }
            i += 1;
        }
    }
}

fn flash_step(grid: &mut [[u8; 10]; 10]) -> usize {
    // keep track
    let mut flashes = 0;

    // increment all energy levels
    grid.iter_mut().flat_map(|row| row.iter_mut()).for_each(|x| *x += 1);

    loop {
        let f = flashes;

        for y in 0..10 {
            for x in 0..10 {
                if grid[y][x] > 9 && grid[y][x] < 128 {
                    grid[y][x] = 128; // an octopus can only flash once
                    flashes += 1;

                    // update all valid neighbours
                    // branch predictor sadness
                    if x > 0 {
                        grid[y][x - 1] += 1;
                    }
                    if x < 9 {
                        grid[y][x + 1] += 1;
                    }
                    if y > 0 {
                        grid[y - 1][x] += 1;
                    }
                    if y < 9 {
                        grid[y + 1][x] += 1;
                    }
                    if x > 0 && y > 0 {
                        grid[y - 1][x - 1] += 1;
                    }
                    if x < 9 && y < 9 {
                        grid[y + 1][x + 1] += 1;
                    }
                    if x > 0 && y < 9 {
                        grid[y + 1][x - 1] += 1;
                    }
                    if x < 9 && y > 0 {
                        grid[y - 1][x + 1] += 1;
                    }
                }
            }
        }

        // if the number of flashes didn't change, let's exit
        if f == flashes {
            break;
        }
    }

    grid.iter_mut()
        .flat_map(|row| row.iter_mut())
        .filter(|x| **x > 9)
        .for_each(|x| *x = 0);

    flashes
}

#[cfg(test)]
mod tests {
    use super::Day11;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn parse() {
        let output = Day11::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day11::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 1656);
    }

    #[test]
    fn part_two() {
        let output = Day11::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 195);
    }
}
