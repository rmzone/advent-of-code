use crate::parse_input;
use common::custom_error::{Error, Result};
use std::iter::zip;

pub fn process(input: &str) -> Result<String, Error> {
    // parse input into two vectors
    let (_, (mut left, mut right)) = parse_input(input).expect("should parse");

    // sort vectors ascending
    left.sort();
    right.sort();

    // calculate sum of differences
    let iter = zip(left, right);
    let sum = iter.map(|(l, r)| (l - r).abs()).sum::<i32>();

    Ok(sum.to_string())
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
