use crate::{count_paths, parse_input};
use common::custom_error::Result;
use std::collections::{HashMap, HashSet, VecDeque};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut programs) = parse_input(input)?;
    let result = count_groups(&mut programs);

    Ok(result.to_string())
}

fn count_groups(programs: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut group_count = 0;

    let mut items = programs.keys().cloned().collect::<Vec<i32>>();

    while !items.is_empty() {
        group_count += 1;

        count_paths(
            *items.get(0).unwrap_or(&0),
            programs,
            &mut queue,
            &mut visited,
        );
        items.retain(|x| !visited.contains(x));
        visited.clear();
    }

    group_count
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
