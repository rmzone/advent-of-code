use crate::has_duplicates;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let mut index = 0;
    let len = input.len();

    for i in 13..len {
        if !has_duplicates(&input[i - 13..=i]) {
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
        assert_eq!("19", process(input)?);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!("23", process(input)?);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!("23", process(input)?);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!("29", process(input)?);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!("26", process(input)?);

        Ok(())
    }
}
