pub mod part1;
pub mod part2;

use common::parsing::Span;
use glam::IVec2;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom_locate::position;
use std::collections::HashMap;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = one_of("XMAS")(input)?;

    Ok((input, (IVec2::new(x, y), c)))
}

pub fn process_input(input: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (input, grid) = separated_list1(line_ending, many1(token))(input)?;
    let hashmap = grid.into_iter().flatten().collect::<HashMap<IVec2, char>>();

    Ok((input, hashmap))
}
