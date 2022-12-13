use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;
use std::iter::zip;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Val(u8),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if is_right_order(self, other).unwrap() {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else if is_right_order(self, other).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

fn main() -> Result<()> {
    println!("Day 13)");
    let input = include_str!("../inputs/input.txt");

    let packet_pairs = parse_input(input)?;

    let answer_1: usize = packet_pairs
        .iter()
        .map(|(packet_1, packet_2)| is_right_order(packet_1, packet_2).unwrap())
        .enumerate()
        .filter(|(_, res)| *res)
        .map(|(i, _)| i + 1)
        .sum();

    println!("\t1) {answer_1}");

    let mut packets = packet_pairs
        .iter()
        .flat_map(|(p1, p2)| vec![p1.to_owned(), p2.to_owned()])
        .collect::<Vec<Packet>>();

    let distress_1 = Packet::List(vec![Packet::List(vec![Packet::Val(2)])]);
    let distress_2 = Packet::List(vec![Packet::List(vec![Packet::Val(6)])]);
    packets.push(distress_1.clone());
    packets.push(distress_2.clone());

    packets.sort();

    let answer_2: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| *packet == &distress_1 || *packet == &distress_2)
        .map(|(i, _)| i+1)
        .product();

    println!("\t2) {answer_2}");

    Ok(())
}

/// Checks if packet_1 (left) and packet_2 (right) are in the correct order
fn is_right_order(packet_1: &Packet, packet_2: &Packet) -> Option<bool> {
    match (packet_1, packet_2) {
        (Packet::Val(v1), Packet::Val(v2)) => {
            if v1 == v2 {
                None
            } else {
                Some(v1 < v2)
            }
        }
        (Packet::List(l1), Packet::List(l2)) => {
            for (elem1, elem2) in zip(l1.iter(), l2.iter()) {
                if let Some(res) = is_right_order(elem1, elem2) {
                    return Some(res);
                }
            }
            if l1.len() == l2.len() {
                None
            } else {
                Some(l1.len() < l2.len())
            }
        }
        (Packet::List(_), Packet::Val(v2)) => {
            is_right_order(packet_1, &Packet::List(vec![Packet::Val(*v2)]))
        }
        (Packet::Val(v1), Packet::List(_)) => {
            is_right_order(&Packet::List(vec![Packet::Val(*v1)]), packet_2)
        }
    }
}

// Parsing code shamelessly stolen from
// https://github.com/litpho/aoc-2022/blob/main/day13/src/main.rs
fn parse(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(pair(line_ending, line_ending), parse_pair)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_tuple(0), line_ending, parse_tuple(0))(input)
}

fn parse_tuple(depth: isize) -> impl Fn(&str) -> IResult<&str, Packet> {
    move |input: &str| map(alt((parse_empty, parse_non_empty(depth))), Packet::List)(input)
}

fn parse_non_empty(depth: isize) -> impl Fn(&str) -> IResult<&str, Vec<Packet>> {
    move |input: &str| {
        delimited(
            complete::char('['),
            separated_list1(complete::char(','), parse_value(depth)),
            complete::char(']'),
        )(input)
    }
}

fn parse_empty(input: &str) -> IResult<&str, Vec<Packet>> {
    map(tag("[]"), |_| vec![Packet::List(vec![])])(input)
}

fn parse_value(depth: isize) -> impl Fn(&str) -> IResult<&str, Packet> {
    move |input: &str| alt((parse_tuple(depth + 1), map(complete::u8, Packet::Val)))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<(Packet, Packet)>> {
    let (_, input) = parse(input)?;

    Ok(input)
}
