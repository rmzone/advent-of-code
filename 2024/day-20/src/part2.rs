use crate::{process_input, DIRECTIONS};
use common::custom_error::{Error, Result};
use common::parsing::Span;
use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::*;
use std::collections::HashSet;

pub fn process(input: &str, time: usize, cheat_time: usize) -> Result<String, Error> {
    let (_, cells) = process_input(Span::new(input)).expect("invalid input");

    // extract start, end, map of walls
    let (start, _) = cells
        .iter()
        .find(|(_, &value)| value == 'S')
        .expect("Should have a start position");

    let (end, _) = cells
        .iter()
        .find(|(_, &value)| value == 'E')
        .expect("Should have a end position");

    let walls = cells
        .iter()
        .filter_map(|(pos, value)| (value == &'#').then_some(pos))
        .cloned()
        .collect::<HashSet<IVec2>>();

    let (original_path, original_cost) = run_track(start, end, &walls, None);
    let original_cost = original_cost as usize;

    // For the solution we don't need to run with all the possible paths. just use the original path
    // and calculate all the different differences.
    let result = original_path
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter_map(|((start_cost, &start_pos), (end_cost, &end_pos))| {
            let distance: usize = (start_pos - end_pos).abs().element_sum() as usize;
            if distance > cheat_time {
                return None;
            };
            let cheat_cost = start_cost + distance + original_cost - end_cost;
            Some(original_cost - cheat_cost)
        })
        .filter(|savings| savings >= &time)
        .count();

    Ok(result.to_string())
}

fn run_track(
    start: &IVec2,
    end: &IVec2,
    walls: &HashSet<IVec2>,
    cheat: Option<&IVec2>,
) -> (Vec<IVec2>, i32) {
    let result = dijkstra(
        start,
        |&position| {
            DIRECTIONS
                .iter()
                .filter_map(|direction| {
                    let next_position = position + *direction;

                    if walls.contains(&next_position)
                        && (cheat.is_none() || cheat.is_some_and(|i| i != &next_position))
                    {
                        None
                    } else {
                        Some((next_position, 1))
                    }
                })
                .collect::<Vec<_>>()
        },
        |&pos| pos == *end,
    )
    .expect("Should work!");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("285", process(input, 50, 20)?);
        Ok(())
    }
}
