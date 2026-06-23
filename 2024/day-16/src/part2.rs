use crate::{process_input, Map};
use common::custom_error::{Error, Result};
use common::parsing::Span;
use glam::IVec2;
use pathfinding::prelude::*;
use std::collections::HashSet;

pub fn process(input: &str) -> Result<String, Error> {
    let (_, Map { start, end, walls }) = process_input(Span::new(input)).expect("Should parse!");

    let (result, _cost) = astar_bag(
        &(start, IVec2::X),
        |(position, direction)| {
            let next_position = *position + *direction;
            if walls.contains(&next_position) {
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                vec![
                    ((next_position, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |_| 0,
        |&(pos, _)| pos == end,
    )
    .expect("Should work!");

    let set = result
        .flat_map(|path| path.into_iter().map(|(position, _)| position))
        .collect::<HashSet<IVec2>>();

    Ok(set.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("45", process(input)?);
        Ok(())
    }
}
