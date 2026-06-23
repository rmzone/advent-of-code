use crate::process_input;
use common::custom_error::Result;
use glam::I64Vec2;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, red_tiles) = process_input(input)?;

    // build line segments
    let mut segments = red_tiles
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&I64Vec2, &I64Vec2)>>();

    // Now let's do the work
    let result = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            (a, b, area)
        })
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, area)| {
            // segment must be outside of the current rectangle
            segments.iter().all(|(start, end)| {
                let left = a.x.max(b.x) <= start.x.min(end.x);
                let right = a.x.min(b.x) >= start.x.max(end.x);
                let top = a.y.max(b.y) <= start.y.min(end.y);
                let bottom = a.y.min(b.y) >= start.y.max(end.y);

                left || right || top || bottom
            })
        });

    Ok(result.unwrap().2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("24", process(input)?);
        Ok(())
    }
}
