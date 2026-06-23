use nom::Parser;
use glam::I64Vec2;
use nom::character::complete;
use nom::character::complete::{char, line_ending};
use nom::multi::separated_list1;
use nom::IResult;

pub mod part1;
pub mod part2;

pub fn process_input(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    let (input, boxes) = separated_list1(line_ending, process_line).parse(input)?;
    Ok((input, boxes))
}

fn process_line(input: &str) -> IResult<&str, I64Vec2> {
    let (input, v) = separated_list1(char(','), complete::i64).parse(input)?;
    Ok((input, I64Vec2::from_slice(&v)))
}
