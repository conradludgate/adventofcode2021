use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Day21([u8; 2]);

impl<'i> ChallengeParser<'i> for Day21 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        // "Player 1 starting position: ".len() == 28
        let bytes = input.as_bytes();
        let a = bytes[28] - b'0';
        let b = bytes[28 + 2 + 28] - b'0';
        Ok((&input[60..], Day21([a, b])))
    }
}

impl Challenge for Day21 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(mut self) -> usize {
        let mut scores = [0, 0];
        let mut turns = 0;
        let mut roll = 6; // dice starts off at 1+2+3
        loop {
            let i = turns % 2;
            let (pos, score) = play(self.0[i], scores[i], roll);
            self.0[i] = pos;
            scores[i] = score;

            // Dice first rolls, a + a+1 + a+2 = (3a + 3).
            // After then rulls a+3 + a+4 + a+5 = (3a + 12).
            // Difference of 9 each turn.
            roll += 9;
            roll %= 10;
            turns += 1;

            if score >= 1000 {
                break scores[turns % 2] * turns * 3;
            }
        }
    }

    fn part_two(self) -> usize {
        let [a, b] = self.count([0, 0]);
        a.max(b)
    }
}

impl Day21 {
    fn count(self, scores: [usize; 2]) -> [usize; 2] {
        let mut counts = [0, 0];

        for (roll, count) in MOVES {
            let [a, b] = scores;
            let Self([x, y]) = self;
            let (x, a) = play(x, a, roll);
            if a >= 21 {
                counts[0] += count;
            } else {
                // swap scores/positions for next player to be 'player 1'
                let scores = [b, a];
                let pos = Self([y, x]);

                // similarly swap outcome counts
                let [b, a] = pos.count(scores);
                counts[0] += a * count;
                counts[1] += b * count;
            }
        }

        counts
    }
}

/// hard coded list of possible scores after 3 dice rolls
/// along with their respecitve distributions across the 3^3 dice rolls
const MOVES: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play(mut pos: u8, mut score: usize, roll: u8) -> (u8, usize) {
    pos += roll;
    pos %= 10;

    score += pos as usize;
    if pos == 0 {
        score += 10; // edge case
    }
    (pos, score)
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
