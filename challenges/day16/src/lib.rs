#![feature(array_chunks)]
use aoc::{Challenge, Parser as ChallengeParser};
use bitvec::{order::Msb0, prelude::BitVec, slice::BitSlice};
use nom::{character::complete::one_of, IResult, Parser};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day16(BitVec<u8, Msb0>);

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
        let (packet, _) = Packet::parse(&self.0);
        packet.sum_ver()
    }

    fn part_two(self) -> usize {
        let (packet, _) = Packet::parse(&self.0);
        packet.eval()
    }
}

#[derive(Debug)]
struct Packet {
    ver: u8,
    typ: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Sum(Vec<Packet>),
    Prod(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Lit(BitVec<u8, Msb0>),
    Gt(Vec<Packet>),
    Lt(Vec<Packet>),
    Eq(Vec<Packet>),
}

impl Packet {
    fn sum_ver(&self) -> usize {
        self.ver as usize
            + match &self.typ {
                PacketType::Lit(_) => 0,
                PacketType::Sum(packets)
                | PacketType::Prod(packets)
                | PacketType::Min(packets)
                | PacketType::Max(packets)
                | PacketType::Gt(packets)
                | PacketType::Lt(packets)
                | PacketType::Eq(packets) => packets.iter().map(Packet::sum_ver).sum::<usize>(),
            }
    }

    fn eval(&self) -> usize {
        match &self.typ {
            PacketType::Lit(lit) => lit.iter().fold(0, |v, b| v << 1 | (*b as usize)),
            PacketType::Sum(packets) => packets.iter().map(Packet::eval).sum(),
            PacketType::Prod(packets) => packets.iter().map(Packet::eval).product(),
            PacketType::Min(packets) => packets.iter().map(Packet::eval).min().unwrap(),
            PacketType::Max(packets) => packets.iter().map(Packet::eval).max().unwrap(),
            PacketType::Gt(packets) => (packets[0].eval() > packets[1].eval()) as usize,
            PacketType::Lt(packets) => (packets[0].eval() < packets[1].eval()) as usize,
            PacketType::Eq(packets) => (packets[0].eval() == packets[1].eval()) as usize,
        }
    }

    fn parse(bits: &BitSlice<u8, Msb0>) -> (Self, &BitSlice<u8, Msb0>) {
        let (ver, bits) = read_u8(bits, 3);
        let (typ, bits) = read_u8(bits, 3);

        let (typ, bits) = match typ {
            0 => parse_op(bits, PacketType::Sum),
            1 => parse_op(bits, PacketType::Prod),
            2 => parse_op(bits, PacketType::Min),
            3 => parse_op(bits, PacketType::Max),
            4 => parse_lit(bits),
            5 => parse_op(bits, PacketType::Gt),
            6 => parse_op(bits, PacketType::Lt),
            7 => parse_op(bits, PacketType::Eq),
            _ => unreachable!(),
        };
        (Self { ver, typ }, bits)
    }
}

fn parse_lit(bits: &BitSlice<u8, Msb0>) -> (PacketType, &BitSlice<u8, Msb0>) {
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

    (PacketType::Lit(litvec), bits)
}

fn parse_op(
    bits: &BitSlice<u8, Msb0>,
    typ: impl FnOnce(Vec<Packet>) -> PacketType,
) -> (PacketType, &BitSlice<u8, Msb0>) {
    let (i, bits) = bits.split_at(1);
    if *i.first().unwrap() {
        parse_fixed_len_op(bits, typ)
    } else {
        parse_fixed_size_op(bits, typ)
    }
}

fn parse_fixed_len_op(
    bits: &BitSlice<u8, Msb0>,
    typ: impl FnOnce(Vec<Packet>) -> PacketType,
) -> (PacketType, &BitSlice<u8, Msb0>) {
    let (l, mut bits) = read_u16(bits, 11);
    let mut packets = vec![];
    for _ in 0..l {
        let (p, b) = Packet::parse(bits);
        packets.push(p);
        bits = b;
    }

    (typ(packets), bits)
}

fn parse_fixed_size_op(
    bits: &BitSlice<u8, Msb0>,
    typ: impl FnOnce(Vec<Packet>) -> PacketType,
) -> (PacketType, &BitSlice<u8, Msb0>) {
    let (l, bits) = read_u16(bits, 15);
    let (mut bits1, bits) = bits.split_at(l as usize);
    let mut packets = vec![];
    while !bits1.is_empty() {
        let (p, b) = Packet::parse(bits1);
        packets.push(p);
        bits1 = b;
    }

    (typ(packets), bits)
}

fn read_u8(bits: &BitSlice<u8, Msb0>, n: usize) -> (u8, &BitSlice<u8, Msb0>) {
    let (v, bits) = bits.split_at(n);
    let v = v.iter().fold(0, |v, b| v << 1 | (*b as u8));
    (v, bits)
}

fn read_u16(bits: &BitSlice<u8, Msb0>, n: usize) -> (u16, &BitSlice<u8, Msb0>) {
    let (v, bits) = bits.split_at(n);
    let v = v.iter().fold(0, |v, b| v << 1 | (*b as u16));
    (v, bits)
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
        assert_eq!(output.part_two(), 46);
    }
}
