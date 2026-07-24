use common::custom_error::Result;
use log::info;
use common::parsing::Span;
use crate::parse_input;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, map) = parse_input(Span::new(input))?;

    // get start and direction down

    info!("{:?}", map);

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";
        assert_eq!("ABCDE", process(input)?);
        Ok(())
    }
}
