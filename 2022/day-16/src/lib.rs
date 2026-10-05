use std::collections::{HashMap, HashSet};
use nom::character::complete::{alpha1, line_ending, space1};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;

pub mod part1;
pub mod part2;

pub type DistanceGrid = Vec<Vec<u32>>;

/// Compute the shortest distances from each valve to every other valve.
/// Use Floyd Warshall Algorithm
pub fn get_distance_map(valves: &[Valve], valve_map: &HashMap<&str, usize>) -> DistanceGrid {
    let mut edges: Vec<Vec<usize>> = vec![ vec![]; valves.len() ];

    for (index, valve) in valves.iter().enumerate() {
        for tunnel in &valve.tunnels {
            edges[index].push(valve_map[tunnel]);
        }
    }

    let mut grid: DistanceGrid = vec![ vec![0; valves.len()]; valves.len()];

    // run dijkstra for each valve with that valve as the starting point of the search
    for (index, valve) in valves.iter().enumerate() {
        let distances = dijkstra(&edges, index);
        grid[index] = distances;
    }

    grid
}

/// Dijkstra's shortest path algorithm.
///
/// Start at `start` and use `distance_array` to track the current shortest distance
/// to each node. This implementation can be simplified as the cost is always 1.
pub fn dijkstra(
    adj_list: &[Vec<usize>],
    start: usize,
) -> Vec<u32> {
    let mut distance_to: Vec<u32> = (0..adj_list.len()).map(|_| u32::MAX).collect();

    // queue up every vertex
    let mut queue: HashSet<usize> = (0..adj_list.len()).collect();

    // We're at `start`, with a zero cost
    distance_to[start] = 0;

    while !queue.is_empty() {
        // find the position in the queue with shortest distance from the starting valve
        let shortest = *queue.iter().min_by(|&&a, &&b| distance_to[a].cmp(&distance_to[b])).unwrap();
        queue.remove(&shortest);

        // get all valves adjacent to the starting one that are still in the queue
        let neighbors: Vec<usize> = adj_list[shortest].iter().filter(|a| queue.contains(a)).cloned().collect();

        for neighbor in neighbors {
            let alt = distance_to[shortest] + 1;

            if alt < distance_to[neighbor] {
                distance_to[neighbor] = alt;
            }
        }
    }

    distance_to
}

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    rate: u32,
    tunnels: Vec<&'a str>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(line_ending, valve).parse(input)
}

fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve")(input)?;
    let (input, name) = delimited(space1, alpha1, space1).parse(input)?;
    let (input, _) = tag("has flow rate=")(input)?;
    let (input, rate) = nom::character::complete::u32.parse(input)?;
    let (input, _) = alt((
                             tag("; tunnels lead to valves "),
                              tag("; tunnel leads to valve ")
                             )).parse(input)?;
    let (input, tunnels) = separated_list0(tag(", "), alpha1).parse(input)?;

    Ok((input, Valve { name, rate, tunnels }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![1, 2], // node0
            vec![0, 4], // node1
            vec![0, 3], // node2
            vec![2, 4], // node3
            vec![3, 1], // node4
        ];

        let result = dijkstra(&graph, 0);
        println!("{:?}", result);
    }
}
