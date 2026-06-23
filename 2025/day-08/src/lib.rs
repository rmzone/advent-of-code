use nom::Parser;
use glam::IVec3;
use nom::character::complete;
use nom::character::complete::{char, line_ending};
use nom::multi::separated_list1;
use nom::IResult;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, Vec<IVec3>> {
    let (input, boxes) = separated_list1(line_ending, process_box).parse(input)?;
    Ok((input, boxes))
}

fn process_box(input: &str) -> IResult<&str, IVec3> {
    let (input, v) = separated_list1(char(','), complete::i32).parse(input)?;
    Ok((input, IVec3::from_slice(&v)))
}
