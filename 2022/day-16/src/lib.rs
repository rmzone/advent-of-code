extern crate core;

use std::collections::HashMap;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq)]
pub enum ValveState {
    Closed,
    Open,
}

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    rate: i32,
    state: ValveState,
    tunnels: Vec<&'a str>,
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Valve>> {
    let (input, valves) = separated_list1(line_ending, valve).parse(input)?;
    let valve_map = valves.into_iter().collect::<HashMap<&str, Valve>>();

    Ok((input, valve_map))
}

fn valve(input: &str) -> IResult<&str, (&str, Valve)> {
    let (input, _) = tag("Valve")(input)?;
    let (input, name) = delimited(space1, alpha1, space1).parse(input)?;
    let (input, _) = tag("has flow rate=")(input)?;
    let (input, rate) = nom::character::complete::i32.parse(input)?;
    let (input, _) = alt((
                             tag("; tunnels lead to valves "),
                              tag("; tunnel leads to valve ")
                             )).parse(input)?;
    let (input, tunnels) = separated_list0(tag(", "), alpha1).parse(input)?;

    Ok((input, (name, Valve { name, rate, state: ValveState::Closed, tunnels })))
}
