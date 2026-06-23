use common::custom_error::Error;
use common::parsing::{token, Span};
use glam::IVec2;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom_locate::position;
use std::collections::HashSet;

pub mod part1;
pub mod part2;

pub struct Map {
    pub start: IVec2,
    pub end: IVec2,
    pub walls: HashSet<IVec2>,
}

pub fn process_input(input: Span) -> common::custom_error::Result<(Span, Map), Error> {
    let (input, cells) = separated_list1(line_ending, many1(token("#.SE")))(input).expect("Should parse!");
    let (start, _) = cells
        .iter()
        .flatten()
        .find(|(_, value)| value == &'S')
        .cloned()
        .expect("Should have a start position");
    let (end, _) = cells
        .iter()
        .flatten()
        .find(|(_, value)| value == &'E')
        .cloned()
        .expect("Should have a end position");
    let walls = cells
        .into_iter()
        .flatten()
        .filter_map(|(pos, value)| (value == '#').then_some(pos))
        .collect::<HashSet<IVec2>>();

    Ok((input, Map { start, end, walls }))
}
