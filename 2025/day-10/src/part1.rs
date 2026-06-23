use crate::{parse_input, Machine};
use common::custom_error::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, machines) = parse_input(input)?;

    let result = machines
        .iter()
        .map(|machine| calculate_pushes(machine))
        .sum::<usize>();

    Ok(result.to_string())
}

/*
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
One way to do this is by pressing  (note xor so every second push is useless)
(0,2) once            1010
(0,1) once            1100
*/

fn calculate_pushes(machine: &Machine) -> usize {
    let bits = machine.joltages.len();
    // info!("{:08b}, {}", &machine.lights, &bits);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0)); // no lights lit and no buttons pushed

    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(0);

    while let Some((lights, count)) = queue.pop_front() {
        // println!("{:08b}", &lights);
        if lights == machine.lights {
            // println!("Count: {}", count);
            return count;
        }

        for neighbor in &machine.buttons {
            let button = neighbor
                .iter()
                .fold(lights, |acc, n| acc ^ (1usize << (bits - n - 1)));

            if visited.insert(button) {
                // println!("{:08b} {:08b} {:?} {}", &lights, &button, &neighbor, &count);
                queue.push_back((button, count + 1));
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
