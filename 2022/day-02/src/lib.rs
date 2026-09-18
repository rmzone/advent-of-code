use nom::character::complete::{alpha1, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::cmp::Ordering;
use std::str::FromStr;

pub mod part1;
pub mod part2;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
pub struct Strategy {
    player1: Shape,
    player2: Shape,
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Shape::Scissors && other == &Shape::Rock {
            Some(Ordering::Less)
        } else if self == &Shape::Rock && other == &Shape::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl Strategy {
    pub fn new(player1: Shape, player2: Shape) -> Strategy {
        Strategy { player1, player2 }
    }

    pub fn calculate_score(&self) -> u32 {
        let player2: u32 = self.player2 as u32;

        if self.player1 > self.player2 {
            return player2 + 0;
        }

        if self.player1 < self.player2 {
            return player2 + 6;
        }

        player2 + 3
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Strategy>> {
    separated_list1(line_ending, strategy).parse(input.trim())
}

fn strategy(input: &str) -> IResult<&str, Strategy> {
    let (input, (a, b)) = separated_pair(alpha1, space1, alpha1).parse(input)?;
    let player1 = a.parse::<Shape>().unwrap();
    let player2 = b.parse::<Shape>().unwrap();
    Ok((input, Strategy { player1, player2 }))
}
