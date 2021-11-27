use std::path::Path;

pub trait Challenge: Sized {
    const NAME: &'static str;

    fn new(input: String) -> Self;

    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;

    fn run() {
        let file = Path::new("challenges").join(Self::NAME).join("input.txt");
        let input = std::fs::read_to_string(file).expect("could not read file");
        let challenge = Self::new(input);
        println!("\nRunning challenge {}", Self::NAME);
        println!("\tAnswer to part one: {}", challenge.part_one());
        println!("\tAnswer to part two: {}\n", challenge.part_two());
    }
}
