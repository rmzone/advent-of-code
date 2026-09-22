use glam::IVec2;
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::{line_ending, one_of, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::collections::{HashMap, HashSet};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Bridge {
    knots: HashMap<usize, IVec2>,
    size: usize,
    visited: HashSet<IVec2>,
}

impl Bridge {
    pub fn new(knot_count: usize) -> Bridge {
        Bridge {
            knots: vec![IVec2::ZERO; knot_count]
                .into_iter()
                .enumerate()
                .collect(),
            size: knot_count,
            visited: HashSet::from([IVec2::ZERO]),
        }
    }

    pub fn move_head(&mut self, command: &Command) {
        for _ in 0..command.steps {
            let head = self.knots.get_mut(&0).unwrap();
            match command.direction {
                Direction::Left => {
                    head.x -= 1;
                }
                Direction::Right => {
                    head.x += 1;
                }
                Direction::Up => {
                    head.y += 1;
                }
                Direction::Down => {
                    head.y -= 1;
                }
            }

            self.adjust_tails();
        }
    }

    fn adjust_tails(&mut self) {
        (0..self.size).tuple_windows().for_each(|(a, b)| {
            self.adjust_tail(a, b);
        });
    }

    fn adjust_tail(&mut self, knot_a: usize, knot_b: usize) {
        let head = *self.knots.get(&knot_a).unwrap();
        let mut tail = *self.knots.get(&knot_b).unwrap();
        let mut distance = IVec2::distance_squared(head, tail);

        while distance > 2 {
            let delta = head - tail;
            let dx = if delta.x == 0 {
                0
            } else {
                delta.x / delta.x.abs()
            };
            let dy = if delta.y == 0 {
                0
            } else {
                delta.y / delta.y.abs()
            };

            tail.x += dx;
            tail.y += dy;

            *self.knots.get_mut(&knot_b).unwrap() = tail;

            // only track the `last` tail
            if knot_b == self.size - 1 {
                self.visited.insert(tail.clone());
            }

            distance = IVec2::distance_squared(head, tail);
        }
    }

    pub fn visited(&self) -> usize {
        self.visited.len()
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    steps: u8,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, command).parse(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, (direction, steps)) =
        separated_pair(one_of("LRUD"), space1, complete::u8).parse(input)?;
    let direction = match direction {
        'L' => Direction::Left,
        'R' => Direction::Right,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => unreachable!(),
    };

    Ok((input, Command { direction, steps }))
}
