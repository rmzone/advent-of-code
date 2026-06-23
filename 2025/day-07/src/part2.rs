use crate::{parse_input, Type};
use common::custom_error::Result;
use common::parsing::Span;
use glam::IVec2;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_input(Span::new(input))?;
    let start = map.find_start();
    let mut beams = HashMap::new();
    beams.insert(start, 1u64);
    let mut row = start.y;

    while row < map.height {
        let mut new_beams = HashMap::new();
        for (beam, count) in beams {
            let mut beam = beam;
            beam.y += 1;
            let loc = map.data.get(&beam);
            if loc.is_some() && loc.unwrap() == &Type::Splitter {
                new_beams
                    .entry(IVec2::new(beam.x - 1, beam.y))
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                new_beams
                    .entry(IVec2::new(beam.x + 1, beam.y))
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            } else {
                new_beams
                    .entry(beam)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        }

        beams = new_beams;
        row += 1;
    }

    let result = beams.iter().map(|(_, u)| u).sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
