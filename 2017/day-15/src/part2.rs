use crate::{generator, parse_input};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, (mut seed_a, mut seed_b)) = parse_input(input)?;

    let mut result = 0;

    for _ in 0..5_000_000 {
        loop {
            seed_a = generator(seed_a, 16807);
            if seed_a % 4 == 0 {
                break;
            }
        }

        loop {
            seed_b = generator(seed_b, 48271);
            if seed_b % 8 == 0 {
                break;
            }
        }

        if seed_a & 0xffff == seed_b & 0xffff {
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
        let input = "Generator A starts with 65
Generator B starts with 8921";
        assert_eq!("309", process(input)?);
        Ok(())
    }
}
