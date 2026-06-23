use common::parsing::{token, Span};
use glam::IVec2;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;
use tracing::info;

pub mod part1;
pub mod part2;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    End,
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<IVec2, char>
}

impl Map {
    /// Find start position in the map looking for the first `|` character
    /// in the top row of the map.
    pub fn find_start(&self) -> Option<IVec2> {
        if let Some((k, _)) = self.map.iter().find(|(k, v)| k.y == 0 && **v == '|') {
            return Some(IVec2::new(k.x, k.y));
        }

        None
    }

    pub fn current(&self, position: &IVec2) -> Option<char> {
        if let Some((_, v)) = self.map.iter().find(|(k, _)| k == &position) {
            return Some(v.clone());
        }

        None
    }

    pub fn next_direction(
        &self,
        position: &IVec2,
        last_direction: &Direction,
    ) -> Option<Direction> {
        // todo: don't go back on oneself
        if last_direction == &Direction::East || last_direction == &Direction::West {
            if let Some(_) = self.map.get(&(position + IVec2::new(0, -1))) {
                return Some(Direction::North);
            }

            if let Some(_) = self.map.get(&(position + IVec2::new(0, 1))) {
                return Some(Direction::South);
            }
        } else if last_direction == &Direction::North || last_direction == &Direction::South {
            if let Some(_) = self.map.get(&(position + IVec2::new(1, 0))) {
                return Some(Direction::East);
            }

            if let Some(_) = self.map.get(&(position + IVec2::new(-1, 0))) {
                return Some(Direction::West);
            }
        }

        None
    }

    pub fn follow_tube(
        &self,
        position: &IVec2,
        direction: &Direction,
        letters: &mut Vec<char>,
    ) -> (IVec2, Direction) {
        // move the player
        let offset = match &direction {
            Direction::North => IVec2::new(0, -1),
            Direction::South => IVec2::new(0, 1),
            Direction::East => IVec2::new(1, 0),
            Direction::West => IVec2::new(-1, 0),
            _ => IVec2::new(-1, -1),
        };

        let next_position = *position + offset;
        let current = self.current(&next_position);
        if current.is_none() {
            return (next_position, Direction::End);
        }

        // accumulate letters
        let cp = current.unwrap();
        info!("{:?}", cp);
        if cp.is_alphabetic() {
            letters.push(cp);
            return (next_position, *direction);
        }

        // only change direction when encountering a `+`
        if cp == '+' {
            let next_direction = self.next_direction(&next_position, &direction);
            if let Some(next_direction) = next_direction {
                return (next_position, next_direction);
            } else {
                return (next_position, Direction::End);
            }
        }

        (next_position, *direction)
    }
}

pub fn parse_input(input: Span) -> IResult<Span, Map> {
    let (input, cells) =
        separated_list1(line_ending, many1(token(" +-|ABCDEFGHIJKLMNOPQRSTUVWXYZ")))
            .parse(input)?;

    let map = cells
        .into_iter()
        .flatten()
        .filter(|(_, value)| value != &' ')
        .collect::<HashMap<IVec2, char>>();

    Ok((input, Map { map }))
}
