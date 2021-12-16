#![feature(array_chunks)]
use aoc::{Challenge, Parser as ChallengeParser};
use bitvec::{
    order::{Lsb0, Msb0},
    prelude::BitVec,
    slice::BitSlice,
};
use nom::{character::complete::one_of, IResult, Parser};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day16(BitVec<Msb0, u8>);

impl<'i> ChallengeParser<'i> for Day16 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        one_of("0123456789ABCDEF")
            .map(|c| c.to_digit(16).unwrap() as u8)
            .many1()
            .map(|h| h.array_chunks().map(|&[x, y]| x << 4 | y).collect())
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Day16 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let (packet, _) = parse_packet(&self.0);
        packet.sum_ver()
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

enum Packet {
    Lit { ver: u8, val: BitVec<Msb0, u8> },
    Op { ver: u8, vals: Vec<Packet> },
}

impl Packet {
    fn sum_ver(&self) -> usize {
        match self {
            Packet::Lit { ver, .. } => *ver as usize,
            Packet::Op { ver, vals } => *ver as usize + vals.iter().map(Packet::sum_ver).sum::<usize>(),
        }
    }
}

fn read_u8(bits: &BitSlice<Msb0, u8>, n: usize) -> (u8, &BitSlice<Msb0, u8>) {
    let (v, bits) = bits.split_at(n);
    let v = v.iter().fold(0, |v, b| v << 1 | (*b as u8));
    (v, bits)
}

fn read_u16(bits: &BitSlice<Msb0, u8>, n: usize) -> (u16, &BitSlice<Msb0, u8>) {
    let (v, bits) = bits.split_at(n);
    let v = v.iter().fold(0, |v, b| v << 1 | (*b as u16));
    (v, bits)
}

fn parse_packet(bits: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
    let (v, bits) = read_u8(bits, 3);
    let (t, bits) = read_u8(bits, 3);

    if t == 4 {
        let mut litvec = BitVec::new();

        let mut bits = bits;
        loop {
            let (s, b0) = bits.split_at(1);
            let (v, b1) = b0.split_at(4);
            litvec.extend_from_bitslice(v);
            bits = b1;
            if !s.first().unwrap() {
                break;
            }
        }

        (Packet::Lit { ver: v, val: litvec }, bits)
    } else {
        let (i, bits) = bits.split_at(1);
        if *i.first().unwrap() {
            let (l, mut bits) = read_u16(bits, 11);
            let mut packets = vec![];
            for _ in 0..l {
                let (p, b) = parse_packet(bits);
                packets.push(p);
                bits = b;
            }

            (Packet::Op { ver: v, vals: packets }, bits)
        } else {
            let (l, bits) = read_u16(bits, 15);
            let (mut bits1, bits) = bits.split_at(l as usize);
            let mut packets = vec![];
            while !bits1.is_empty() {
                let (p, b) = parse_packet(bits1);
                packets.push(p);
                bits1 = b;
            }

            (Packet::Op { ver: v, vals: packets }, bits)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day16;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "C0015000016115A2E0802F182340";

    #[test]
    fn parse() {
        let output = Day16::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day16::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 23);
    }

    #[test]
    fn part_two() {
        let output = Day16::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
