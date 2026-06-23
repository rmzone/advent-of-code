use std::collections::BTreeMap;
use common::custom_error::Result;
use crate::{count_paths, parse_input};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, connections) = parse_input(input)?;
    let result = count_paths(&connections, &mut BTreeMap::new(), "you", "out");

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
