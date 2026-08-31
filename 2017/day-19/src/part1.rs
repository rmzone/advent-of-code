use crate::{parse_input, Direction};
use common::custom_error::Result;
use common::parsing::Span;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_input(Span::new(input))?;

    let mut position = map.find_start().unwrap();

    info!("map: {:?}", &map);
    info!("start: {:?}", &position);

    let mut direction = Direction::South;
    let mut letters: Vec<char> = Vec::new();

    while direction != Direction::End {
        let (new_position, new_direction) = map.follow_tube(&position, &direction, &mut letters);
        position = new_position;
        direction = new_direction;
        info!("positon: {:?}, direction: {:?}", &position, &direction);
    }

    Ok(letters.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";
        assert_eq!("ABCDEF", process(input)?);
        Ok(())
    }
}
