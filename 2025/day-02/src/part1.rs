use common::custom_error::Result;
use crate::process_input;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, ranges) = process_input(input)?;
    let mut result: u64 = 0;

    for id in ranges.into_iter().flatten() {
        if is_invalid(id) {
            result += id;
        }
    }

    Ok(result.to_string())
}

// Convert to a string and check if the first half repeats with the second half
// Odd length id's will never be considered as repeated.
fn is_invalid(id: u64) -> bool {
    let id_str = id.to_string();
    if id_str.len().is_power_of_two(){
        return false;
    }

    let half = id_str.len() / 2;

    &id_str[..half] == &id_str[half..]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
