use nom::bytes::complete::tag;
use nom::character::complete;
use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("Generator A starts with ").parse(input)?;
    let (input, seed_a) = complete::u64(input)?;
    let (input, _) = tag("\nGenerator B starts with ").parse(input)?;
    let (input, seed_b) = complete::u64(input)?;
    Ok((input, (seed_a, seed_b)))
}

pub fn generator(seed: u64, factor: u64) -> u64 {
    seed * factor % 0x7fffffff
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_generator() {
        assert_eq!(430625591, generator(8921, 48271));
    }
}
