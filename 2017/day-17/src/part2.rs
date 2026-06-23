use crate::parse_input;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, steps) = parse_input(input)?;
    let mut index = 0usize;
    let mut result = 0i32;

    // We never need the full buffer: only whether the next insertion lands at
    // position 1 (right after 0). If it does, the inserted value becomes the new
    // answer. Otherwise the answer stays the same.
    for v in 1..=50_000_000 {
        index = (index + steps as usize) % v;

        if index == 0 {
            result = v as i32;
        }

        index += 1;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3";
        assert_eq!("1222153", process(input)?);
        Ok(())
    }
}
