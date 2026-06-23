use std::ops::RangeInclusive;
use nom::Parser;
use nom::character::complete::{char, u64};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

pub mod part1;
pub mod part2;

pub fn process_input(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
     separated_list1(char(','), range).parse(input)
}

pub fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(u64, char('-'), u64)
        .map(|(start, end)| start..=end)
        .parse(input)
}
