use crate::parse_input;
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, pairs) = parse_input(input)?;

    info!("{:?}", pairs);

    let count: i32 = pairs
        .iter()
        .map(|u| if u.is_contained() { 1 } else { 0 })
        .sum();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
