use common::custom_error::{Error, Result};
use glam::IVec2;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use pathfinding::prelude::dijkstra;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
pub fn process(input: &str, size: i32) -> Result<String, Error> {
    let (_, coords) = parse_input(input).expect("Should parse!");

    // brut force to find the first coord that will block all paths
    let mut index = 0;

    for i in 0..coords.len() {
        let slice = &coords[..i];

        // println!("{},{}", coords[i].x, coords[i].y);

        // find the shortest path from (0,0) to (size, size). see day-16
        let start = IVec2::new(0, 0);
        let end = IVec2::new(size, size);

        let result = dijkstra(
            &start,
            |position| {
                DIRECTIONS
                    .iter()
                    .filter_map(|direction| {
                        let next_position = *position + *direction;

                        // check map bounds
                        if next_position.x < 0
                            || next_position.x > size
                            || next_position.y < 0
                            || next_position.y > size
                        {
                            return None;
                        }

                        if slice.contains(&next_position) {
                            return None;
                        } else {
                            Some((next_position, 1))
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&pos| pos == end,
        );

        if result.is_none() {
            index = i - 1;
            break;
        }
    }

    let coord = coords[index];
    let output = format!("{},{}", coord.x, coord.y);

    Ok(output.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<IVec2>> {
    let (input, items) = separated_list1(
        line_ending,
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)?;

    let items = items.into_iter().map(|(x, y)| IVec2::new(x, y)).collect();

    Ok((input, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", process(input, 6)?);
        Ok(())
    }
}
