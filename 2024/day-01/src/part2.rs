use crate::parse_input;
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    // parse input into two vectors
    let (_, (left, right)) = parse_input(input).expect("should parse");

    let result: i32 = left
        .iter()
        .map(|l| l * right.iter().filter(|r| &l == r).count() as i32)
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
