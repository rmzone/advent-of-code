use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, i32, line_ending, multispace1, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GateType {
    And,
    Or,
    Xor,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Gate<'a> {
    pub gate_type: GateType,
    pub input1: &'a str,
    pub input2: &'a str,
    pub output: &'a str,
}

pub fn execute(input_a: bool, input_b: bool, gate_type: GateType) -> bool {
    match gate_type {
        GateType::And => input_a & input_b,
        GateType::Or => input_a | input_b,
        GateType::Xor => input_a ^ input_b,
        GateType::None => false,
    }
}

pub fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, bool>, Vec<Gate>)> {
    let (input, (states, gates)) = separated_pair(
        separated_list1(line_ending, parse_state),
        multispace1,
        separated_list1(line_ending, parse_gate),
    )(input)?;

    let states = states.into_iter().collect::<HashMap<&str, bool>>();

    Ok((input, (states, gates)))
}

fn parse_state(input: &str) -> IResult<&str, (&str, bool)> {
    let (input, (signal, value)) = separated_pair(alphanumeric1, tag(": "), i32)(input)?;
    let state = match value {
        0 => false,
        1 => true,
        _ => unreachable!(),
    };

    Ok((input, (signal, state)))
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (input, a) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, op) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, b) = alphanumeric1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, out) = alphanumeric1(input)?;

    let gate_type = match op {
        "AND" => GateType::And,
        "OR" => GateType::Or,
        "XOR" => GateType::Xor,
        _ => unreachable!(),
    };

    Ok((
        input,
        Gate {
            gate_type,
            input1: a,
            input2: b,
            output: out,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let input = "x00: 1";

        assert_eq!(parse_state(input.trim()).unwrap(), ("", ("x00", true)));
    }
}
