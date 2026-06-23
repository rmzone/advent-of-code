use crate::{process_input, DIRECTIONS};
use common::custom_error::{Error, Result};
use common::parsing::Span;
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, HashSet};

pub fn process(input: &str, time: i32) -> Result<String, Error> {
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

    let width = walls.iter().map(|pos| pos.x).max().unwrap();
    let height = walls.iter().map(|pos| pos.y).max().unwrap();
    let base_time = run_track(start, end, &walls, None);

    println!("Base time: {}", &base_time);

    let width = walls.iter().map(|v| v.x).max().unwrap_or(0);
    let height = walls.iter().map(|v| v.y).max().unwrap_or(0);
    let cheats: Vec<IVec2> = walls
        .iter()
        .filter(|&v| v.x > 0 && v.x < width && v.y > 0 && v.y < height)
        .cloned()
        .collect::<Vec<IVec2>>();

    let mut stats: HashMap<i32, i32> = HashMap::new();

    for cheat in cheats.iter() {
        let new_time = run_track(start, end, &walls, Some(cheat));
        // println!("Time: {}", new_time);
        let time_saved = base_time - new_time;
        stats.entry(time_saved).and_modify(|e| *e += 1).or_insert(1);
    }

    dbg!(&stats);

    let result = stats
        .iter()
        .filter(|(&k, _)| k >= time)
        .map(|(_, &v)| v)
        .sum::<i32>();

    Ok(result.to_string())
}

fn run_track(start: &IVec2, end: &IVec2, walls: &HashSet<IVec2>, cheat: Option<&IVec2>) -> i32 {
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

    result.1 as i32
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
        assert_eq!("1", process(input, 64)?);
        Ok(())
    }
}
