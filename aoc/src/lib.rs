use std::{collections::HashMap, path::Path};

use reqwest::header;

const YEAR: usize = 2021;

pub trait Challenge: Sized {
    const NAME: &'static str;

    fn new(input: String) -> Self;

    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;

    fn run() {
        println!("\nRunning challenge {}", Self::NAME);

        let file = Path::new("challenges").join(Self::NAME).join("input.txt");
        let input = std::fs::read_to_string(file).expect("could not read file");

        let challenge = Self::new(input);

        let session = dotenv::var("AOC_SESSION").unwrap();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&format!("session={}", session)).unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let day = Self::NAME[3..].parse::<i32>().unwrap();
        let url = format!("https://adventofcode.com/{}/day/{}/answer", YEAR, day);

        let file = Path::new("challenges").join(Self::NAME).join("README.md");
        let readme = std::fs::read_to_string(file).expect("could not read file");

        if !readme.contains("--- Part Two ---") {
            let p1 = challenge.part_one();
            println!("\tAnswer to part one: {}", p1);
            client
                .post(&url)
                .form(&[("level", 1), ("answer", p1)].into_iter().collect::<HashMap<_, _>>())
                .send()
                .unwrap();
        } else {
            let p2 = challenge.part_two();
            println!("\tAnswer to part two: {}\n", p2);
            client
                .post(&url)
                .form(&[("level", 2), ("answer", p2)].into_iter().collect::<HashMap<_, _>>())
                .send()
                .unwrap();
        }
    }
}
