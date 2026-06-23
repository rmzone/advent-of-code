use crate::{hash_list, knot_hash};
use common::custom_error::Result;
use log::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let mut result = 0;
    let input = input.trim();

    for i in 0..128 {
        let seed = format!("{}-{}", &input, i);
        let lengths = seed.chars().map(|c| c as i32).collect::<Vec<i32>>();
        let list = knot_hash(&lengths);
        let hash = hash_list(&list);
        info!("{} : {}", &hash, i);

        result += count_ones(&hash);
    }

    Ok(result.to_string())
}

fn count_ones(input: &String) -> i32 {
    let mut bits = 0;

    for i in 0..2 {
        let start = 16 * i;
        let end = 15 + 16 * i;
        let chunk = &input[start..=end];
        let mut value = u64::from_str_radix(chunk, 16).unwrap();

        while value > 0 {
            if value & 1 == 1 {
                bits += 1;
            }

            value >>= 1;
        }
    }

    bits
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "flqrgnkx";
        assert_eq!("8108", process(input)?);
        Ok(())
    }
}
