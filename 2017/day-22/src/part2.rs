use crate::{Direction, State, parse_input};
use common::custom_error::Result;
use common::parsing::Span;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut map) = parse_input(Span::new(input))?;
    let mut position = map.find_start();
    let mut direction = Direction::Up;
    let mut infected_count = 0;

    info!("[{:?}]", &position);

    for _ in 0..10000000 {
        let current_node = map.get_node(&position);
        match current_node {
            State::Infected => {
                direction = map.turn_right(direction);
                map.set_node(&position, State::Flagged);
            }
            State::Clean => {
                direction = map.turn_left(direction);
                map.set_node(&position, State::Weakened);
            }
            State::Weakened => {
                map.set_node(&position, State::Infected);
                infected_count += 1
            }
            State::Flagged => {
                direction = map.reverse(direction);
                map.set_node(&position, State::Clean);
            }
        }

        position = map.move_node(&position, &direction);
    }

    Ok(infected_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "..#
#..
...";
        assert_eq!("2511944", process(input)?);
        Ok(())
    }
}
