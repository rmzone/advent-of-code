use common::custom_error::Result;
use crate::{process_input, Direction};

pub fn process(input: &str) -> Result<String> {
    let (_, combinations) = process_input(input).expect("Error while parsing input");

    let mut dial = 50;
    let mut result = 0;

    for combination in &combinations {
        match combination.direction {
            Direction::Left => {
                dial -= 100 + combination.value;
            },
            Direction::Right => {
                dial += combination.value;
            }
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            result += 1;
        }
    }

    Ok(result.to_string())
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
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
