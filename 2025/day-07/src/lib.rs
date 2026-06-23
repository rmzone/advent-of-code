use nom::Parser;
use common::parsing::Span;
use glam::IVec2;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom_locate::position;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Empty,
    Start,
    Splitter,
}

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub data: HashMap<IVec2, Type>,
}

impl Map {
    pub fn find_start(&self) -> IVec2 {
        let start = *self
            .data
            .iter()
            .find(|(_, v)| v == &&Type::Start)
            .expect("Expected to find a Start position")
            .0;

        start
    }
}

pub fn parse_input(input: Span) -> IResult<Span, Map> {
    let (input, cells) = separated_list1(line_ending, many1(process_cell)).parse(input)?;

    let data = cells
        .into_iter()
        .flatten()
        .collect::<HashMap<IVec2, Type>>();

    let width = data.keys().map(|v| v.x).max().unwrap_or(0) + 1;
    let height = data.keys().map(|v| v.y).max().unwrap_or(0) + 1;

    Ok((
        input,
        (Map {
            width,
            height,
            data,
        }),
    ))
}

fn process_cell(input: Span) -> IResult<Span, (IVec2, Type)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, t) = alt((
        value(Type::Empty, complete::char('.')),
        value(Type::Splitter, complete::char('^')),
        value(Type::Start, complete::char('S')),
    )).parse(input)?;

    Ok((input, (IVec2::new(x, y), t)))
}
