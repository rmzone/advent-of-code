use common::parsing::{Span, token};
use glam::IVec2;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};
use std::collections::HashMap;

pub mod part1;
pub mod part2;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum State {
    Infected,
    Clean,
    Weakened,
    Flagged,
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<IVec2, State>,
}


impl Map {
    /// Start at the center of the map
    pub fn find_start(&self) -> IVec2 {
        let width = (self.map.iter().map(|(r, _)| r.x).max().unwrap_or(0)) / 2;
        let height = (self.map.iter().map(|(r, _)| r.y).max().unwrap_or(0)) / 2;

        IVec2::new(width, height)
    }

    pub fn get_node(&mut self, position: &IVec2) -> State {
        let node = self.map.entry(*position).or_insert(State::Clean);
        *node
    }

    pub fn set_node(&mut self, position: &IVec2, state: State) {
        let node = self.map.entry(*position).or_insert(State::Clean);
        *node = state.clone();
    }

    pub fn turn_left(&mut self, direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&mut self, direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn reverse(&mut self, direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn move_node(&mut self, position: &IVec2, direction: &Direction) -> IVec2 {
        match direction {
            Direction::Up => position + IVec2::new(0, -1),
            Direction::Right => position + IVec2::new(1, 0),
            Direction::Down => position + IVec2::new(0, 1),
            Direction::Left => position + IVec2::new(-1, 0),
        }
    }
}

pub fn parse_input(input: Span) -> IResult<Span, Map> {
    let (input, cells) = separated_list1(line_ending, many1(token(".#"))).parse(input)?;

    let map = cells
        .into_iter()
        .flatten()
        .map(|(i, c)| {
            let state = match c {
                '.' => State::Clean,
                '#' => State::Infected,
                _ => State::Clean,
            };
            (i, state)
        })
        .collect::<HashMap<IVec2, State>>();

    Ok((input, Map { map }))
}
