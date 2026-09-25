use crate::{parse_input, Pair};
use common::custom_error::Result;
use std::cmp::Ordering;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, pairs) = parse_input(input)?;
    info!("{:#?}", pairs);

    let result = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, Pair { left, right })| match left.cmp(right) {
            Ordering::Less => Some(idx + 1),
            Ordering::Equal => panic!("Should not be equal"),
            Ordering::Greater => None,
        })
        .sum::<usize>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
