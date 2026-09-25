use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use nom::Parser;
use std::cmp::Ordering;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(left), Self::List(right)) => left == right,
            (Self::List(left), Self::Number(right)) => left == &vec![Self::Number(*right)],
            (Self::Number(left), Self::List(right)) => &vec![Self::Number(*left)] == right,
            (Self::Number(left), Self::Number(right)) => left == right,
        }
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(left), Self::List(right)) => left.cmp(right),
            (Self::List(left), Self::Number(right)) => left.cmp(&vec![Self::Number(*right)]),
            (Self::Number(left), Self::List(right)) => vec![Self::Number(*left)].cmp(&right),
            (Self::Number(left), Self::Number(right)) => left.cmp(right),
        }
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(multispace1, pair).parse(input)
}

fn pair(input: &str) -> IResult<&str, Pair> {
    separated_pair(packet, line_ending, packet)
        .map(|(left, right)| Pair { left, right })
        .parse(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))
    .parse(input)
}
