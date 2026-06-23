use crate::{accessible, process_input, MapType};
use common::custom_error::Result;
use glam::IVec2;
use log::info;
use std::collections::{HashMap, HashSet};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let mut map = process_input(input);

    let mut result = 0;
    let mut done = false;

    while !done {
        let (temp, to_remove) = run(&map);
        info!("result: {}, to_remove_count: {}", temp, to_remove.len());

        if temp <= 0 {
            done = true;
            continue;
        }

        result += temp;

        for elem in &to_remove {
            map.remove(&elem);
        }
    }

    Ok(result.to_string())
}

fn run(map: &HashMap<IVec2, MapType>) -> (i32, HashSet<IVec2>) {
    let mut accessable = HashSet::new();
    let result = map
        .iter()
        .filter(|&(&u, &v)| v == MapType::ToiletPaper)
        .fold(0, |acc, (&u, _)| {
            let amount = accessible(&u, &map);
            if amount.len() < 4 {
                accessable.insert(u);
                return acc + 1;
            } else {
                return acc;
            }
        });

    (result, accessable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
