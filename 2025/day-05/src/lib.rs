use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub mod part1;
pub mod part2;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

pub fn process_input(input: &str) -> IResult<&str, (Vec<Range>, Vec<u64>)> {
    let (input, ranges) = terminated(parse_ranges, line_ending).parse(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = parse_items(input)?;

    Ok((input, (ranges, items)))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(line_ending, parse_range).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(complete::u64, tag("-"), complete::u64).parse(input)?;
    Ok((input, Range { start, end }))
}

fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(line_ending, complete::u64).parse(input)
}
