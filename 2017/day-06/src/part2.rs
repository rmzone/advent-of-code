use crate::{find_first_max, parse_input};
use common::custom_error::Result;
use log::info;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut banks) = parse_input(input)?;
    let mut steps = 0;
    let mut seen: HashMap<Vec<i32>, i32> = HashMap::new();

    while !seen.contains_key(&banks) {
        // find the largest block
        if let Some((mut index, value)) = find_first_max(&banks) {
            seen.insert(banks.clone(), steps);

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

    let cycles = steps - seen.get(&banks).unwrap();

    Ok(cycles.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0 2 7 0";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
