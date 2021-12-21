use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::tag,
    character::streaming::line_ending,
    sequence::{delimited, tuple},
    IResult, Parser,
};
use parsers::number;

#[derive(Debug, PartialEq, Clone)]
pub struct Day21(u8, u8);

impl<'i> ChallengeParser<'i> for Day21 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let player1 = delimited(tag("Player 1 starting position: "), number, line_ending);
        let player2 = delimited(tag("Player 2 starting position: "), number, line_ending);

        tuple((player1, player2)).map(|(x, y)| Day21(x, y)).parse(input)
    }
}

impl Challenge for Day21 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(mut self) -> usize {
        let mut scores = (0, 0);
        let mut dice = (1..=100).cycle();
        let mut rolls = 0;
        loop {
            play(&mut self.0, &mut scores.0, roll(&mut dice));
            rolls += 3;

            if scores.0 >= 1000 {
                break scores.1 * rolls;
            }

            std::mem::swap(&mut scores.0, &mut scores.1);
            std::mem::swap(&mut self.0, &mut self.1);
        }
    }

    fn part_two(self) -> usize {
        let (a, b) = self.count((0, 0));
        a.max(b)
    }
}

impl Day21 {
    fn count(self, scores: (usize, usize)) -> (usize, usize) {
        let mut counts = (0, 0);

        for (roll, count) in MOVES {
            let (mut a, b) = scores;
            let Self(mut x, y) = self;
            play(&mut x, &mut a, roll);
            if a >= 21 {
                counts.0 += count;
            } else {
                let scores = (b, a);
                let pos = Self(y, x);
                let (b, a) = pos.count(scores);
                counts.0 += a * count;
                counts.1 += b * count;
            }
        }

        counts
    }
}

/// hard coded list of possible scores after 3 dice rolls
/// along with their respecitve distributions across the 3^3 dice rolls
const MOVES: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn roll(dice: &mut impl Iterator<Item = u8>) -> u8 {
    dice.next().unwrap() % 10 + dice.next().unwrap() % 10 + dice.next().unwrap() % 10
}

fn play(pos: &mut u8, score: &mut usize, roll: u8) {
    *pos += roll;
    *pos %= 10;

    *score += *pos as usize;
    if *pos == 0 {
        *score += 10; // edge case
    }
}

#[cfg(test)]
mod tests {
    use super::Day21;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn parse() {
        let output = Day21::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day21::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 739785);
    }

    #[test]
    fn part_two() {
        let output = Day21::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 444356092776315);
    }
}
