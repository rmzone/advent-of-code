use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(line_ending, separated_pair(alpha1, tag("-"), alpha1))(input)
}

pub fn build_connection_map<'a>(
    items: Vec<(&'a str, &'a str)>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for item in items {
        if connections.contains_key(item.0) {
            connections.get_mut(item.0).unwrap().insert(item.1);
        } else {
            let mut map = HashSet::new();
            map.insert(item.1);
            connections.insert(item.0, map);
        }

        if connections.contains_key(item.1) {
            connections.get_mut(item.1).unwrap().insert(item.0);
        } else {
            let mut map = HashSet::new();
            map.insert(item.0);
            connections.insert(item.1, map);
        }
    }

    connections
}

pub fn build_connection_map2<'a>(
    items: Vec<(&'a str, &'a str)>,
) -> (HashSet<&'a str>, HashMap<&'a str, HashSet<&'a str>>) {
    let (vertices, edges) = items
        .into_iter() // or into_iter??
        .fold(
            (HashSet::new(), HashMap::new()),
            |(mut vertices, mut edges), (left, right)| {
                vertices.insert(left);
                vertices.insert(right);
                edges.entry(left).or_insert_with(HashSet::new).insert(right);
                edges.entry(right).or_insert_with(HashSet::new).insert(left);

                (vertices, edges)
            },
        );

    (vertices, edges)
}
