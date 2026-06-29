use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::{separated_list0, separated_list1};
use nom::IResult;
use nom::Parser;
use std::collections::{HashMap, HashSet, VecDeque};

pub mod part1;
pub mod part2;

pub fn count_paths(
    id: i32,
    programs: &HashMap<i32, Vec<i32>>,
    queue: &mut VecDeque<i32>,
    visited: &mut HashSet<i32>,
) -> i32 {
    queue.push_back(id);
    visited.insert(id);

    while let Some(child) = queue.pop_front() {
        for neighbour in programs[&child].iter() {
            if !visited.contains(neighbour) {
                queue.push_back(*neighbour);
                visited.insert(*neighbour);
            }
        }
    }

    visited.len() as i32
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<i32, Vec<i32>>> {
    let (input, programs) = separated_list1(line_ending, parse_program).parse(input)?;
    Ok((input, programs.into_iter().collect()))
}

fn parse_program(input: &str) -> IResult<&str, (i32, Vec<i32>)> {
    let (input, id) = complete::i32(input)?;
    let (input, _) = tag(" <-> ")(input)?;
    let (input, connections) = separated_list0(tag(", "), complete::i32).parse(input)?;

    Ok((input, (id, connections)))
}
