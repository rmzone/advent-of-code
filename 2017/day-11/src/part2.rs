use crate::{distance, move_player, parse_input};
use common::custom_error::Result;
use glam::IVec3;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, directions) = parse_input(input)?;

    let mut position = IVec3::ZERO;
    let mut max_distance = 0;

    info!("{:?}", position);

    for direction in directions {
        position = move_player(position, direction);
        let current_distance = distance(IVec3::ZERO, position);
        if current_distance > max_distance {
            max_distance = current_distance;
        }
    }

    Ok(max_distance.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
