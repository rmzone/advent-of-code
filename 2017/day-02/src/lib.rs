extern crate core;

use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32)).parse(input)
}
