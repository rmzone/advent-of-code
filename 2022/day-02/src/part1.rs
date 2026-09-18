use crate::parse_input;
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, rounds) = parse_input(input)?;
    info!("{:?}", &rounds);
    let score = rounds.iter().map(|u| u.calculate_score()).sum::<u32>();

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "A Y
B X
C Z";
        assert_eq!("15", process(input)?);
        Ok(())
    }
}
