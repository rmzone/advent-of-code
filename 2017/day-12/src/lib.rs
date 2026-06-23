use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::{separated_list0, separated_list1};
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Program {
    pub id: i32,
    pub connections: Vec<i32>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Program>> {
    separated_list1(line_ending, parse_program).parse(input)
}

fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, id) = complete::i32(input)?;
    let (input, _) = tag(" <-> ")(input)?;
    let (input, connections) = separated_list0(tag(", "), complete::i32).parse(input)?;

    Ok((input, Program { id, connections }))
}
