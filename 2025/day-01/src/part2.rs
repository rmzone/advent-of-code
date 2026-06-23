use common::custom_error::Result;
use log::info;
use crate::{process_input, Direction};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, combinations) = process_input(input)?;

    let mut dial = 50;
    let mut result = 0;

    for combination in &combinations {
        let (new_dial, clicks) = turn_dial(&combination.direction, combination.value, dial);
        result += clicks;
        dial = new_dial;
    }

    Ok(result.to_string())
}

fn turn_dial(direction: &Direction, value: i32, dial: i32) -> (i32, i32) {
    let mut new_dial = dial;

    match direction {
        Direction::Left => {
            new_dial -= value;
        },
        Direction::Right => {
            new_dial += value;
        },
    }

    info!("Combination: {:?}{}, Dial: {} -> {} ({})", direction, value, dial, new_dial, new_dial.rem_euclid(100));

    let mut clicks = (new_dial / 100).abs();

    if dial != 0 && new_dial <= 0 {
        clicks += 1;
    }

    new_dial = new_dial.rem_euclid(100);

    info!("Clicks: {}", clicks);

    (new_dial, clicks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_edge_case1() {
        let (new_dial, clicks) = turn_dial(&Direction::Right, 1000, 50);

        assert_eq!(50, new_dial);
        assert_eq!(10, clicks);
    }

    #[test]
    fn test_edge_case2() {
        let (new_dial, clicks) = turn_dial(&Direction::Left, 1000, 50);

        assert_eq!(50, new_dial);
        assert_eq!(10, clicks);
    }
}
