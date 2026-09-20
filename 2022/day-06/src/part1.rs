use crate::has_duplicates;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let mut index = 0;
    let len = input.len();

    for i in 3..len {
        if !has_duplicates(&input[i - 3..=i]) {
            index = i + 1;
            break;
        }
    }

    Ok(index.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!("7", process(input)?);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!("5", process(input)?);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!("6", process(input)?);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!("10", process(input)?);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!("11", process(input)?);

        Ok(())
    }
}
