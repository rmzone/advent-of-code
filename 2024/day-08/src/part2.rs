use crate::process_input;
use common::custom_error::{Error, Result};
use glam::IVec2;
use itertools::Itertools;
use std::collections::HashSet;
use common::parsing::Span;

pub fn process(input: &str) -> Result<String, Error> {
    let columns = input.lines().next().unwrap().len() as i32;
    let rows = input.lines().count() as i32;
    let (_, map) = process_input(Span::new(input)).expect("should parse!");

    // find all pairs and create a map of anti nodes
    let mut antinodes: HashSet<IVec2> = HashSet::new();

    for nodes in map.values() {
        let node = nodes.iter().combinations(2);
        for pair in node {
            make_antinode(&mut antinodes, pair[0], pair[1], columns, rows);
        }
    }

    // add the nodes
    for nodes in map.values() {
        for node in nodes {
            antinodes.insert(*node);
        }
    }

    let result = antinodes.len();

    Ok(result.to_string())
}

fn make_antinode(antinodes: &mut HashSet<IVec2>, p1: &IVec2, p2: &IVec2, width: i32, height: i32) {
    let dx = (p2.x - p1.x);
    let dy = (p2.y - p1.y);

    // loop in each direction until we are off the map
    let mut last_point = p1.clone();
    loop {
        let a1 = IVec2::new(last_point.x - dx, last_point.y - dy);
        if a1.x >= 0 && a1.x < width && a1.y >= 0 && a1.y < height {
            antinodes.insert(a1);
            last_point = a1.clone();
        } else {
            break;
        }
    }

    last_point = p2.clone();
    loop {
        let a2 = IVec2::new(last_point.x + dx, last_point.y + dy);
        if a2.x >= 0 && a2.x < width && a2.y >= 0 && a2.y < height {
            antinodes.insert(a2);
            last_point = a2.clone();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
