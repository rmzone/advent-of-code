use crate::calculate_priority;
use common::custom_error::Result;
use itertools::Itertools;
use std::collections::HashSet;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let lines = input.lines().chunks(3);
    let mut total = 0;

    for chunk in &lines {
        let intersection = chunk.fold(HashSet::new(), |mut acc, line| {
            let current = line.chars().collect::<HashSet<char>>();
            if acc.len() == 0 {
                acc = current;
            } else {
                acc = current.intersection(&acc).cloned().collect();
            }

            acc
        });

        let combo = intersection.iter().collect::<Vec<_>>();
        total += calculate_priority(&combo);
    }

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
        assert_eq!("70", process(input)?);
        Ok(())
    }
}
