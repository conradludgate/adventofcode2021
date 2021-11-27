use aoc::Challenge;

struct Day00(String);

impl Challenge for Day00 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn new(input: String) -> Self {
        Self(input)
    }

    fn part_one(&self) -> usize {
        todo!()
    }

    fn part_two(&self) -> usize {
        todo!()
    }
}

fn main() {
    Day00::run()
}
