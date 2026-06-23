use crate::parse_input;
use common::custom_error::Result;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, pass_phrases) = parse_input(input)?;

    let result = pass_phrases
        .iter()
        .fold(0, |acc, phrase| acc + is_valid(&phrase));

    Ok(result.to_string())
}

fn is_valid(phrase: &Vec<&str>) -> i32 {
    let count = phrase.iter().combinations(2).fold(0, |acc, comb| {
        if comb[0] == comb[1] {
            return acc + 1;
        }

        acc
    });

    if count > 0 { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
