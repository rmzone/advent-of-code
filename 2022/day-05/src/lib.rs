use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, digit1, line_ending, multispace1, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use std::collections::VecDeque;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Cargo<'a> {
    stacks: Vec<VecDeque<&'a str>>,
}

impl<'a> Cargo<'a> {
    pub fn new(stacks: Vec<VecDeque<&'a str>>) -> Self {
        Self { stacks }
    }

    pub fn move_crates(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.move_count {
            let crt = self.stacks[instruction.from as usize - 1]
                .pop_front()
                .unwrap();
            self.stacks[instruction.to as usize - 1].insert(0, crt);
        }
    }

    pub fn move_many_crates(&mut self, instruction: &Instruction) {
        let mut queue = vec![];
        for _ in 0..instruction.move_count {
            let crt = self.stacks[instruction.from as usize - 1]
                .pop_front()
                .unwrap();
            queue.push(crt);
        }

        for item in queue.iter().rev() {
            self.stacks[instruction.to as usize - 1].insert(0, item);
        }
    }

    pub fn find_top(&self) -> String {
        let mut top = String::new();
        for item in &self.stacks {
            top.push((*item.front().unwrap()).parse().unwrap());
        }

        top
    }
}

#[derive(Debug)]
pub struct Instruction {
    move_count: u8,
    from: u8,
    to: u8,
}

pub fn parse_input(input: &str) -> IResult<&str, (Cargo<'_>, Vec<Instruction>)> {
    let (input, horizontal_stacks) = separated_list1(line_ending, horizontal_stack).parse(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = many1(preceded(space1, digit1)).parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, instructions) = separated_list1(line_ending, instructions).parse(input)?;

    // convert horizontal_stacks to stacks
    let mut stacks = vec![];
    let len = horizontal_stacks.last().unwrap().len();
    for _ in 0..len {
        stacks.push(VecDeque::new());
    }

    for stack in horizontal_stacks {
        for (index, item) in stack.iter().enumerate() {
            if let Some(item) = item {
                stacks[index].push_back(*item);
            }
        }
    }

    Ok((input, (Cargo::new(stacks), instructions)))
}

fn horizontal_stack(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), parse_crate).parse(input)
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))
    .parse(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn instructions(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, move_count, _, from, _, to)) = ((
        tag("move "),
        complete::u8,
        tag(" from "),
        complete::u8,
        tag(" to "),
        complete::u8,
    ))
        .parse(input)?;

    Ok((
        input,
        Instruction {
            move_count,
            from,
            to,
        },
    ))
}
