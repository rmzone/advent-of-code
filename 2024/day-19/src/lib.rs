use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, multispace1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(tag(", "), alpha1),
        multispace1,
        separated_list1(line_ending, alpha1),
    )(input)
}
