use nom::Parser;
use std::collections::HashMap;
use glam::{IVec2, IVec3};
use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use common::parsing::{token, Span};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    End
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<IVec2, char>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn find_start() -> IVec2 {
        IVec2::new(0, 0)
    }

    // pass by ref?
    pub fn follow_tube(position: IVec2, direction: Direction) -> (IVec2, Direction) {
        let offset = match direction {
            Direction::North => IVec2::new(0, -1),
            Direction::South => IVec2::new(0, 1),
            Direction::East => IVec2::new(1, 0),
            Direction::West => IVec2::new(-1, 0),
            _ => IVec2::new(-1, -1),
        };

        // todo: bounds check to see if we are done or there is no more
        // pass next dierection
        (position + offset, Direction::South)
    }
}

pub fn parse_input(input: Span) -> IResult<Span, Map> {
    let (input, cells) = separated_list1(line_ending, many1(token(" +-|ABCDEFGHIJKLMNOPQRSTUVWXYZ")))
        .parse(input)?;

    let map = cells
        .into_iter()
        .flatten()
        .filter(|(pos, value)| (value != &' '))
        .collect::<HashMap<IVec2, char>>();

    let width = map.keys().map(|v| v.x).max().unwrap_or(0) + 1;
    let height = map.keys().map(|v| v.y).max().unwrap_or(0) + 1;

    Ok((input, Map { map, width, height }))
}
