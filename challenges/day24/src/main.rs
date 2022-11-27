use aoc::{Challenge, Parser};
use day24::Day24;

fn main() {
    let input = aoc::load::<Day24>();
    // aoc::run::<Day24>(&input);
    let challenge = Day24::parse(&input).unwrap().1;
    dbg!(challenge.clone().part_one());
    dbg!(challenge.part_two());
}
