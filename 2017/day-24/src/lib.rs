use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Port {
    pub type1: i32,
    pub type2: i32,
}

pub fn calculate_strength(ports: &[Port]) -> i32 {
    ports
        .iter()
        .fold(0, |acc, port| acc + (port.type1 + port.type2))
}

pub fn my_display(ports: &Vec<Port>) -> String {
    ports
        .iter()
        .map(|p| format!("{}/{}", p.type1, p.type2))
        .join("--")
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Port>> {
    separated_list1(line_ending, port).parse(input)
}

fn port(input: &str) -> IResult<&str, Port> {
    map(separated_pair(i32, tag("/"), i32), |(a, b)| Port {
        type1: a,
        type2: b,
    })
    .parse(input)
}
