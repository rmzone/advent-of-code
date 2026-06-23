use common::custom_error::Result;
use glam::IVec2;
use nom::character::complete::{line_ending, satisfy};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use nom_locate::{position, LocatedSpan};
use std::collections::{HashMap, HashSet};

pub type Span<'a> = LocatedSpan<&'a str>; // trick to simplify usage

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn process_cell(input: Span) -> IResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_numeric())(input)?;

    Ok((input, (IVec2::new(x, y), c.to_digit(10).unwrap())))
}

fn process_input(input: Span) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, lines) = separated_list1(line_ending, many1(process_cell))(input)?;

    let hashmap = lines
        .iter()
        .flatten()
        .copied()
        .collect::<HashMap<IVec2, u32>>();

    Ok((input, hashmap))
}

fn traverse(start: &IVec2, map: &HashMap<IVec2, u32>) -> u32 {
    let mut locations = HashSet::from([*start]);
    let mut trail_ends = HashSet::new();
    let mut depth = 1;

    while !locations.is_empty() && depth <= 9 {
        let mut new_locations = HashSet::new();

        for location in locations.iter() {
            for direction in DIRECTIONS.iter() {
                let position = *location + *direction;
                let &d = map.get(&position).unwrap_or(&0);

                if d == depth {
                    new_locations.insert(position);

                    if d == 9 {
                        trail_ends.insert(position);
                    }
                }
            }
        }

        locations = new_locations;
        depth += 1;
    }

    trail_ends.len() as u32
}

pub fn process(input: &str) -> Result<String> {
    let (_, map) = process_input(Span::new(input)).expect("Should parse!");
    let trail_heads = map
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(x, _)| x)
        .collect::<Vec<_>>();
    let mut results = 0;

    for &start in trail_heads.iter() {
        let temp = traverse(start, &map);
        results += temp;
    }

    Ok(results.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
