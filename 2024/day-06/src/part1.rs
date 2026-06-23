use crate::{move_guard, process_input, MapType};
use common::custom_error::{Error, Result};
use glam::{IVec2, Vec2Swizzles};

pub fn process(input: &str) -> Result<String, Error> {
    let columns = input.lines().next().unwrap().len() as i32;
    let rows = input.lines().count() as i32;
    let mut map = process_input(input);
    let mut position = *map
        .iter()
        .find(|(_, &map_type)| map_type == MapType::Guard)
        .unwrap()
        .0;
    let mut direction = IVec2::new(0, -1);
    let mut can_move = true;

    // println!("{} : {}", &position, &direction);

    while can_move {
        // println!("{} : {}", &position, &direction);

        match map.get_mut(&position) {
            Some(value) => *value = MapType::Visited,
            None => println!("Key does not exist in the HashMap."),
        }
        (can_move, position, direction) = move_guard(position, direction, &map, rows, columns);
    }

    let count = map
        .iter()
        .filter(|(_, &map_type)| map_type == MapType::Visited)
        .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
