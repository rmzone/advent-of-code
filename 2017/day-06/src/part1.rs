use crate::{find_first_max, parse_input};
use common::custom_error::Result;
use log::info;
use std::collections::HashSet;

/*
- each round find the bank with the most blocks (tie goes to lowest bank)
- then remove all blocks and round robin place 1 block until no more blocks
- is_balanced condition if the state is the same as the last round
*/

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut banks) = parse_input(input)?;
    let mut steps = 0;
    let mut seen: HashSet<Vec<i32>> = HashSet::new();

    while !seen.contains(&banks) {
        // find the largest block
        if let Some((mut index, value)) = find_first_max(&banks) {
            seen.insert(banks.clone());

            info!("{:?} {} {}", &banks, &index, &value);

            // rebalance items
            let mut items = banks[index];
            banks[index] = 0;

            while items > 0 {
                index += 1;
                index %= banks.len();
                banks[index] += 1;
                items -= 1;
            }
        }

        steps += 1;
    }

    info!("Final bank: {:?}", &banks);

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0 2 7 0";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
