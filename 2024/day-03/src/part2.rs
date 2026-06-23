use crate::{parse_input, Instruction};
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, instructions) = parse_input(input).expect("should parse");

    let (_, count) = instructions
        .iter()
        .fold((true, 0), |(enabled, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if enabled {
                    (enabled, acc + a * b)
                } else {
                    (enabled, acc)
                }
            }
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
        });

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
