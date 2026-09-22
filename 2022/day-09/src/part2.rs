use crate::{Bridge, parse_input};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, commands) = parse_input(input)?;

    info!("{:?}", commands);

    let mut bridge = Bridge::new(10);

    for command in commands {
        bridge.move_head(&command);
    }

    Ok(bridge.visited().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!("1", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> Result<()> {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
