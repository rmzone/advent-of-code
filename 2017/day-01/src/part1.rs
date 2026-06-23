use crate::parse_input;
use common::custom_error::Result;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let captcha = parse_input(input);

    let result = captcha
        .iter()
        .circular_tuple_windows()
        .fold(0, |acc, (a, b)| acc + if a == b { *a } else { 0 });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("3", process("1122")?);
        assert_eq!("4", process("1111")?);
        assert_eq!("0", process("1234")?);
        assert_eq!("9", process("91212129")?);

        Ok(())
    }
}
