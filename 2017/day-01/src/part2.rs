use crate::parse_input;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let captcha = parse_input(input);

    // for part 2 we need a circular window but the next item is half way around
    let mut result = 0;
    let len: usize = captcha.len() / 2;

    for index in 0usize..captcha.len() {
        let a = captcha[index];
        let b = captcha[(index + len) % captcha.len()];

        if a == b {
            result += a;
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
        assert_eq!("6", process("1212")?);
        assert_eq!("0", process("1221")?);
        assert_eq!("4", process("123425")?);
        assert_eq!("12", process("123123")?);
        assert_eq!("4", process("12131415")?);

        Ok(())
    }
}
