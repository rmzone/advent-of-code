use nom::Parser;
use nom::character::complete::{i32, line_ending, one_of};
use nom::IResult;
use nom::multi::separated_list1;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right
}

#[derive(Debug)]
pub struct Combination {
    pub direction: Direction,
    pub value: i32
}

pub fn process_input(input: &str) -> IResult<&str, Vec<Combination>> {
    separated_list1(line_ending, process_combination).parse(input)
}

pub fn process_combination(input: &str) -> IResult<&str, Combination> {
    let (input, direction) = one_of("LR")(input)?;
    let (input, value) = i32(input)?;

    let direction = match direction {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!()
    };

    Ok((input, Combination { direction, value }))
}
