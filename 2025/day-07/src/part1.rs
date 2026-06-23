use crate::{parse_input, Type};
use common::custom_error::Result;
use common::parsing::Span;
use glam::IVec2;
use std::collections::HashSet;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_input(Span::new(input))?;
    let start = map.find_start();
    let mut beams = HashSet::new();
    beams.insert(start);
    let mut splits = 0;
    let mut row = start.y;

    while row < map.height {
        let mut new_beams = HashSet::new();
        for beam in beams {
            let mut beam = beam;
            beam.y += 1;
            let loc = map.data.get(&beam);
            if loc.is_some() && loc.unwrap() == &Type::Splitter {
                splits += 1;
                new_beams.insert(IVec2::new(beam.x - 1, beam.y));
                new_beams.insert(IVec2::new(beam.x + 1, beam.y));
            } else {
                new_beams.insert(beam);
            }
        }

        beams = new_beams;
        row += 1;
    }

    Ok(splits.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
