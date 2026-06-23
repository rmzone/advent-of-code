use crate::{count_paths, parse_input};
use common::custom_error::Result;
use std::collections::{HashSet, VecDeque};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut programs) = parse_input(input)?;

    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut visited: HashSet<i32> = HashSet::new();

    let result = count_paths(0, &mut programs, &mut queue, &mut visited);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
