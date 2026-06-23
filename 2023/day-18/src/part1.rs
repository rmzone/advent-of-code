use std::collections::{HashMap, HashSet};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, line_ending
    },
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult, Parser,
};
use nom::character::complete::{hex_digit1, space1};
use common::custom_error::AocError;
use glam::IVec2;
use itertools::{Itertools, MinMaxResult};

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[derive(Debug, Clone, Copy)]
struct Command {
    direction: IVec2,
    count: u32,
    color: Color
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, direction) = alt((
        complete::char('R').map(|_| IVec2::X),
        complete::char('L').map(|_| IVec2::NEG_X),
        complete::char('U').map(|_| IVec2::Y),
        complete::char('D').map(|_| IVec2::NEG_Y),
    ))(input)?;
    let (input, count) = delimited(space1, complete::u32, space1)(input)?;
    let (input, hex) = delimited(tag("(#"), hex_digit1, complete::char(')'))(input)?;
    let color =  Color {r:0,g:0,b:0}; // todo:

    Ok((input, Command {direction, count, color}))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, command)(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, commands) = parse_input(input).unwrap();

    // dbg!(commands);

    let mut holes: Vec<IVec2> = vec![];
    let mut position = IVec2::new(0, 0);

    // loop over all commands and dig
    for command in &commands {
        for _i in 0..command.count {
            position += command.direction;
            holes.push(position.clone());
        }
    }

    // get extant of matrix
    let MinMaxResult::MinMax(x_min, x_max) =
        holes.iter().map(|pos| pos.x).minmax()
        else {
            panic!("should have a min and max for x");
        };

    let MinMaxResult::MinMax(y_min, y_max) =
        holes.iter().map(|pos| pos.y).minmax()
        else {
            panic!("should have a min and max for y");
        };

    // dbg!(x_min, x_max, y_min, y_max);

    // perform fill

    // result is how many holes dug up
    let result = holes.iter().count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
