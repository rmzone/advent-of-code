use crate::{distance, move_player, parse_input};
use common::custom_error::Result;
use glam::IVec3;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, directions) = parse_input(input)?;

    // let result = directions.iter().fold(IVec3::ZERO, |acc, direction| {
    //
    // });

    let mut position = IVec3::ZERO;

    info!("{:?}", position);

    for direction in directions {
        position = move_player(position, direction);
        info!("{:?}", position);
    }

    let result = distance(IVec3::ZERO, position);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "se,sw,se,sw,sw";
        assert_eq!("3", process(input)?); // s,s,sw -1, 3
        Ok(())
    }
}
