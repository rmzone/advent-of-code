use crate::calculate_priority;
use common::custom_error::Result;
use std::collections::HashSet;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let lines = input.lines();
    let total = lines.fold(0, |acc, line| {
        let half = line.len() / 2;
        let left = line[..half].chars().collect::<HashSet<char>>();
        let right = line[half..].chars().collect::<HashSet<char>>();
        let intersection = left.intersection(&right).collect::<Vec<_>>();

        info!(
            "left:{:?} right:{:?}, inter: {:?}",
            left, right, intersection
        );

        let priority = calculate_priority(&intersection);

        acc + priority
    });

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!("157", process(input)?);
        Ok(())
    }
}
