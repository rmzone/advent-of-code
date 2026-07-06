use crate::{hash_list, knot_hash};
use common::custom_error::Result;
use glam::IVec2;
use std::collections::{HashSet, VecDeque};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let map = decode_hash(&input);
    // display(&map);

    let mut visited: HashSet<IVec2> = HashSet::new();

    let result: i32 = map.iter().map(|&v| find_group(v, &map, &mut visited)).sum();

    Ok(result.to_string())
}

const NORTH: IVec2 = IVec2::new(0, -1);
const SOUTH: IVec2 = IVec2::new(0, 1);
const EAST: IVec2 = IVec2::new(1, 0);
const WEST: IVec2 = IVec2::new(-1, 0);

fn find_group(start: IVec2, map: &Vec<IVec2>, visited: &mut HashSet<IVec2>) -> i32 {
    if visited.contains(&start) {
        return 0;
    }

    let mut queue: VecDeque<IVec2> = VecDeque::new();

    visited.insert(start);
    queue.push_back(start.clone());

    while let Some(child) = queue.pop_front() {
        let neighbors = map
            .iter()
            .filter(|&u| {
                *u == child + NORTH
                    || *u == child + SOUTH
                    || *u == child + EAST
                    || *u == child + WEST
            })
            .map(|u| u.clone())
            .collect::<Vec<IVec2>>();

        for neighbour in &neighbors {
            if !visited.contains(neighbour) {
                queue.push_back(*neighbour);
                visited.insert(*neighbour);
            }
        }
    }

    1
}

fn display(map: &Vec<IVec2>) {
    for row in 0..128 {
        for col in 0..128 {
            if map.iter().find(|u| u.x == col && u.y == row).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn decode_hash(input: &str) -> Vec<IVec2> {
    let mut map = Vec::new();
    // remove any trailing whitespace
    let input = input.trim();

    for i in 0..128 {
        let seed = format!("{}-{}", &input, i);
        let lengths = seed.chars().map(|c| c as i32).collect::<Vec<i32>>();
        let list = knot_hash(&lengths);
        let hash = hash_list(&list);
        let row = expand(&hash);
        // println!("{} {}", &hash, &row);

        let mut data = vectorize(&row, i);
        map.append(&mut data);
    }

    map
}

fn vectorize(input: &str, row: i32) -> Vec<IVec2> {
    let mut map = Vec::new();

    let mut col = 0;
    for item in input.chars() {
        if item == '#' {
            map.push(IVec2::new(col, row));
        }

        col += 1;
    }

    map
}

fn expand(input: &String) -> String {
    let mut bits = String::new();

    for i in (0..2).rev() {
        let start = 16 * i;
        let end = 15 + 16 * i;
        let chunk = &input[start..=end];
        let mut value = u64::from_str_radix(chunk, 16).unwrap();

        // while value > 0 {
        for _ in 0..64 {
            if value & 1 == 1 {
                bits.push('#');
            } else {
                bits.push('.');
            }

            value >>= 1;
        }
    }

    bits.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "wenycdww"; //"flqrgnkx";
        assert_eq!("1242", process(input)?);
        Ok(())
    }
}
