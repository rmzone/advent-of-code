use common::parsing::Span;
use glam::IVec2;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom_locate::position;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

pub fn process_input(input: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (input, cells) = separated_list1(line_ending, many1(token))(input).expect("Should parse!");

    let walls = cells
        .into_iter()
        .flatten()
        .collect::<HashMap<IVec2, char>>();

    Ok((input, walls))
}

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = one_of("#.SE")(input)?;

    Ok((input, (IVec2::new(x, y), c)))
}
