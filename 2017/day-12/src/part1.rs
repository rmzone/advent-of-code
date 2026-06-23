use crate::{parse_input, Program};
use common::custom_error::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, programs) = parse_input(input)?;
    let mut cache: HashMap<i32, i32> = HashMap::new();

    let result = programs
        .iter()
        .map(|program| count_paths(program, &mut cache))
        .sum::<i32>();

    Ok(result.to_string())
}

// path counting from each node to `0`
// dp with memo
// if we already parsed a node then save if it is connected to `0`
// We assume this is a DAG without cycles
// use dynamic programming to find the count of paths from the current node to the '0'.
// Each nodes path is the sum of the paths of its neighbors.
pub fn count_paths(program: &Program, cache: &mut HashMap<i32, i32>) -> i32 {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new(); // parent, child node

    for child in &program.connections {
        queue.push_back((program.id, *child));
    }

    while let Some((parent, child)) = queue.pop_front() {

        //     if lights == machine.lights {
        //         // println!("Count: {}", count);
        //         return count;
        //     }

        //     for neighbor in &machine.buttons {
        //         let button = neighbor
        //             .iter()
        //             .fold(lights, |acc, n| acc ^ (1usize << (bits - n - 1)));
        //
        //         if visited.insert(button) {
        //             // println!("{:08b} {:08b} {:?} {}", &lights, &button, &neighbor, &count);
        //             queue.push_back((button, count + 1));
        //         }
        //     }
    }

    0
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
