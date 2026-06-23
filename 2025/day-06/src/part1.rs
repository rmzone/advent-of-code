use crate::process_input;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, problems) = process_input(input)?;
    let result = problems.iter().map(|p| p.calculate()).sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "123 328  51 64
45 64  387 23
6 98  215 314
1 0  1  0
*   +   *   +";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
