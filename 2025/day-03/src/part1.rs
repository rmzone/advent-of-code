use common::custom_error::Result;
use crate::Bank;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let banks = input.lines().map(|u| Bank::from_str(u)).collect::<Vec<Bank>>();
    let result = banks.iter().map(|u| u.max_voltage()).sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
