use std::collections::BTreeMap;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

pub mod part1;
pub mod part2;

// We know it is a DAG without cycles
// use dynamic programming to find the count of paths from the current node to the end.
// Each nodes path is the sum of the paths of it's neighbors.
pub fn count_paths<'a>(
    connections: &BTreeMap<&'a str, Vec<&'a str>>,
    paths: &mut BTreeMap<&'a str, usize>,
    cur: &'a str,
    end: &str
) -> usize {
    // If we counted the paths for this node before, no need to do it again.
    if let Some(p) = paths.get(cur) {
        return *p;
    }

    // If we reached the end then we found one path
    if cur == end {
        return 1;
    }

    // Count the paths of the nearest neighbors
    let total = match connections.get(cur) {
        Some(neighbours) => { neighbours.iter().map(|&next| count_paths(connections, paths, next, end)).sum() }
        None => { 0 }
    };

    // Memorize the results
    paths.insert(cur, total);

    total
}

pub fn parse_input(input: &str) -> IResult<&str, BTreeMap<&str, Vec<&str>>> {
    let (input, items) = separated_list1(line_ending, connections).parse(input)?;
    let map: BTreeMap<&str, Vec<&str>> = items.into_iter().collect();

    Ok((input, map))
}

fn connections(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), operators).parse(input)
}

fn operators(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alpha1).parse(input)
}
