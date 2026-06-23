use common::custom_error::Result;
use crate::{process_input, Range};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, (ranges, items)) = process_input(input)?;
    let result = items.iter().filter(|&item| in_range(*item, &ranges)).count();

    Ok(result.to_string())
}

fn in_range(id: u64, ranges: &Vec<Range>) -> bool {
    for range in ranges {
        if range.start <= id && id <= range.end {
            return true
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
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
