use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, terminated};
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct JumpInstruction<'a> {
    register: &'a str,
    operation: Operation,
    value: i32,
    condition_lhs: &'a str,
    condition_type: Condition,
    condition_rhs: i32,
}

#[derive(Debug)]
pub enum Operation {
    Inc,
    Dec,
}

#[derive(Debug)]
pub enum Condition {
    GreaterThan,
    LessThan,
    EqualTo,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    NotEqualTo,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<JumpInstruction<'_>>> {
    separated_list1(line_ending, parse_instruction).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, JumpInstruction<'_>> {
    let (input, register) = terminated(alpha1, space1).parse(input)?;
    let (input, operation) = terminated(parse_operation, space1).parse(input)?;
    let (input, value) = terminated(complete::i32, space1).parse(input)?;
    let (input, condition_lhs) = delimited(tag("if "), alpha1, space1).parse(input)?;
    let (input, condition_type) = terminated(parse_condition, space1).parse(input)?;
    let (input, condition_rhs) = complete::i32.parse(input)?;

    Ok((
        input,
        JumpInstruction {
            register,
            operation,
            value,
            condition_lhs,
            condition_type,
            condition_rhs,
        },
    ))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(alpha1, |s| match s {
        "inc" => Operation::Inc,
        "dec" => Operation::Dec,
        _ => unreachable!(),
    })
    .parse(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    alt((
        map(tag(">="), |_| Condition::GreaterThanOrEqualTo),
        map(tag("<="), |_| Condition::LessThanOrEqualTo),
        map(tag("=="), |_| Condition::EqualTo),
        map(tag("!="), |_| Condition::NotEqualTo),
        map(tag(">"), |_| Condition::GreaterThan),
        map(tag("<"), |_| Condition::LessThan),
    ))
    .parse(input)
}
