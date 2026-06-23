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

// We need to check if the id has two or more repeating digits.
// Example: 565656
// We have six digits. Candidates to check for repeat are:
// 5   -> 555555 ❌
// 56  -> 565656 ✅
// 565 -> 565565 ❌
fn is_invalid(id: u64) -> bool {
    let digits = u64::ilog10(id) + 1;

    for i in 1..=(digits / 2) {
        if !digits.is_multiple_of(i) {
            continue;
        }

        let prefix = id / 10u64.pow(digits - i);
        let candidate = (1..digits / i).fold(prefix, | acc, _ | acc * 10u64.pow(i) + prefix);
        if id == candidate {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(565656));
    }
}
