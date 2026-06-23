use common::custom_error::Result;
use itertools::Itertools;
use nom::character::complete::{line_ending, satisfy};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom_locate::{position, LocatedSpan};
use petgraph::visit::IntoNeighbors;
use petgraph::{algo::condensation, dot::Dot, prelude::*, visit::IntoNodeReferences};
use std::collections::HashMap;
// Notes:
// https://docs.rs/petgraph/latest/petgraph/
// https://cp-algorithms.com/graph/strongly-connected-components.html
// Also might be more efficient with a flood fill algorithm

pub type Span<'a> = LocatedSpan<&'a str>; // trick to simplify usage
const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

pub fn process(input: &str) -> Result<String> {
    let (_, map) = process_input(Span::new(input)).expect("Should parse!");
    let graph = generate_graph(&map);
    let new_graph = generate_condensed_graph(&graph);

    println!("{:?}", Dot::with_config(&graph, &[]));

    // process the graph
    let mut result = 0usize;

    for (_node_index, node_list) in new_graph.node_references() {
        let group_id = map.get(&node_list[0]).unwrap();
        let area = node_list.len();
        let nb_sides = calculate_side_count(group_id, node_list, &map);
        result += area * nb_sides;
    }

    Ok(result.to_string())
}

fn calculate_side_count(
    group_id: &char,
    node_list: &[(i32, i32)],
    map: &HashMap<(i32, i32), char>,
) -> usize {
    // The number of corners == the number of sides
    let mut count = 0usize;

    for node in node_list.iter() {
        /*
            We only want these combinations (never across)
            [0, 1], [1, 0]
            [1, 0], [0, -1]
            [0, -1], [-1, 0]
            [-1, 0], [0, 1]
        */
        for ([x0, y0], [x1, y1]) in DIRECTIONS.iter().circular_tuple_windows() {
            let test_0 = map
                .get(&(x0 + node.0, y0 + node.1))
                .is_some_and(|c| c == group_id); // [x0, y0]
            let test_1 = map
                .get(&(x1 + node.0, y1 + node.1))
                .is_some_and(|c| c == group_id); // [x1, y1]
            let test_01 = map
                .get(&(x0 + x1 + node.0, y0 + y1 + node.1))
                .is_some_and(|c| c == group_id); // [x0, y0] + [x1, y1]

            if test_0 && test_1 && !test_01 {
                // interior corner
                count += 1;
            } else if !test_0 && !test_1 {
                // exterior corner
                count += 1;
            }
        }

        println!("{}: {:?} {}", &group_id, &node, &count)
    }

    println!("Final {}: {}", &group_id, &count);
    count
}

fn generate_condensed_graph(
    graph: &GraphMap<(i32, i32), (), Undirected>,
) -> Graph<Vec<(i32, i32)>, (), Undirected, NodeIndex> {
    condensation(graph.clone().into_graph::<NodeIndex>(), false)
}

fn generate_graph(map: &HashMap<(i32, i32), char>) -> GraphMap<(i32, i32), (), Undirected> {
    let mut graph = UnGraphMap::<(i32, i32), ()>::new();
    for ((x, y), c) in map.iter() {
        let node = graph.add_node((*x, *y));

        for [x1, y1] in DIRECTIONS.iter() {
            // add edge if there is a map element adjacent
            let new_node = (x + x1, y + y1);

            if map.get(&new_node).is_some_and(|c2| c == c2) {
                graph.add_edge(node, new_node, ());
            }
        }
    }

    graph
}

fn process_input(input: Span) -> IResult<Span, HashMap<(i32, i32), char>> {
    let (input, lines) = separated_list1(line_ending, many1(process_cell))(input)?;

    let hashmap = lines
        .iter()
        .flatten()
        .copied()
        .collect::<HashMap<(i32, i32), char>>();

    Ok((input, hashmap))
}

fn process_cell(input: Span) -> IResult<Span, ((i32, i32), char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_alphanumeric())(input)?;

    Ok((input, ((x, y), c)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() -> Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("80", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("436", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example4() -> Result<()> {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
