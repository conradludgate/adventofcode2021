use std::path::Path;

const YEAR: usize = 2021;

pub trait Parser<'i>: Sized + Challenge {
    fn parse(input: &'i str) -> nom::IResult<&'i str, Self>;
}

pub trait Challenge {
    const NAME: &'static str;

    fn part_one(self) -> usize;
    fn part_two(self) -> usize;
}

pub fn load<C: Challenge>() -> String {
    println!("\nRunning challenge {}", C::NAME);

    let file = Path::new("challenges").join(C::NAME).join("input.txt");
    std::fs::read_to_string(file).expect("could not read file")
}

pub fn run<'i, P: Parser<'i>>(input: &'i str) {
    let challenge = P::parse(input).unwrap().1;

    let file = Path::new("challenges").join(P::NAME).join("README.md");
    let readme = std::fs::read_to_string(file).expect("could not read file");
    let part_one = !readme.contains("--- Part Two ---");

    if part_one {
        let p1 = challenge.part_one();
        println!("\tAnswer to part one: {}", p1);
        submit::<P>(1, p1);
    } else {
        let p2 = challenge.part_two();
        println!("\tAnswer to part two: {}\n", p2);
        submit::<P>(2, p2);
    }
}

fn submit<C: Challenge>(level: usize, answer: usize) {
    let session = dotenv::var("AOC_SESSION").unwrap();

    let day = C::NAME[3..].parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{}/day/{}/answer", YEAR, day);

    ureq::post(&url)
        .set("Cookie", &format!("session={session}"))
        .send_form(&[
            ("level", &format!("{level}")),
            ("answer", &format!("{answer}")),
        ])
        .unwrap();
}
