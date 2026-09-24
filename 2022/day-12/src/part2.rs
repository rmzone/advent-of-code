use crate::parse_input;
use common::custom_error::Result;
use common::parsing::Span;
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let row_count = input.lines().count() as i32;
    let column_count = input.lines().next().unwrap().len() as i32;
    let (_, mut grid) = parse_input(Span::new(input))?;
    let (&start, _) = grid.iter().find(|(_, ch)| **ch == 'S').unwrap();
    let (&end, _) = grid.iter().find(|(_, ch)| **ch == 'E').unwrap();

    // Replace 'S' and 'E'
    *grid.get_mut(&start).unwrap() = 'a';
    *grid.get_mut(&end).unwrap() = 'z';

    info!("{:?} : {:?}", start, end);

    // get all 'a' as multiple starts
    let starts = grid
        .iter()
        .filter(|(_, ch)| **ch == 'a')
        .map(|(pos, _)| *pos)
        .collect::<Vec<IVec2>>();
    let mut path_lengths: Vec<usize> = Vec::new();

    info!("starts: {}", starts.len());

    for start in starts {
        let result: (Vec<IVec2>, usize) = dijkstra(
            &start,
            |position| {
                crate::part1::DIRECTIONS
                    .iter()
                    .filter_map(|direction| {
                        let next_position = *position + *direction;

                        // check map bounds
                        if next_position.x < 0
                            || next_position.x >= column_count
                            || next_position.y < 0
                            || next_position.y >= row_count
                        {
                            return None;
                        }

                        // test if this candidate is a valid successor
                        let current = grid[position] as i32;
                        let next = grid[&next_position] as i32;

                        if current + 1 == next || next <= current {
                            Some((next_position, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&pos| pos == end,
        )
        .unwrap_or((Vec::new(), usize::max_value()));
        path_lengths.push(result.1);
    }

    info!("Path lengths: {:?}", path_lengths);

    let shortest_path_length = path_lengths.iter().min().unwrap();

    Ok(shortest_path_length.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!("29", process(input)?);
        Ok(())
    }
}
