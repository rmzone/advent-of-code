use common::parsing::{Span, token};
use glam::IVec2;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

pub fn parse_input(input: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (input, output) =
        separated_list1(line_ending, many1(token("abcdefghijklmnopqrstuvwxyzSE"))).parse(input)?;

    let grid = output
        .into_iter()
        .flat_map(|v| v.into_iter())
        .collect::<HashMap<IVec2, char>>();

    Ok((input, grid))
}
