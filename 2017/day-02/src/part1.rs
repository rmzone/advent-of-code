use crate::parse_input;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, rows) = parse_input(input)?;

    let result = rows.iter().fold(0, |acc, row| {
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();
        acc + max - min
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
