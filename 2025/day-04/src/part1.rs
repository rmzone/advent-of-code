use crate::{accessible, process_input, MapType};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let map = process_input(input);

    let result = map
        .iter()
        .filter(|&(&u, &v)| v == MapType::ToiletPaper)
        .fold(0, |acc, (&u, _)| {
            let amount = accessible(&u, &map);
            if amount.len() < 4 {
                return acc + 1;
            } else {
                return acc;
            }
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input)?);
        Ok(())
    }

    #[test]
    fn test_edge_case1() -> Result<()> {
        let input = "@@@
@@@
@@@
";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
