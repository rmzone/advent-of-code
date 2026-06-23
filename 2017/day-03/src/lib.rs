use nom::character::complete;
use nom::IResult;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, i32> {
    complete::i32(input)
}
