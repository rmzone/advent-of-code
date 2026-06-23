use crate::{parse_input, Instruction};
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, instructions) = parse_input(input).expect("should parse");

    let count = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<u32>();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
