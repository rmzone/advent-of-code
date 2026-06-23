use crate::{move_guard, process_input, MapType};
use common::custom_error::{Error, Result};
use glam::{IVec2, Vec2Swizzles};
use std::collections::{HashMap, HashSet};

pub fn process(input: &str) -> Result<String, Error> {
    let columns = input.lines().next().unwrap().len() as i32;
    let rows = input.lines().count() as i32;
    let mut map: HashMap<IVec2, MapType> = process_input(input);

    let mut position = *map
        .iter()
        .find(|(_, &map_type)| map_type == MapType::Guard)
        .unwrap()
        .0;
    let mut direction = IVec2::new(0, -1);
    let mut can_move = true;

    // save start position and direction
    // get all open positions
    // add in an obstacle and run to see if we can get the guard stuck in a loop
    // add up all the possibilities where we can loop
    // track visited positions separately.
    // possibilities are where we visited in the first run, excluding the start.  (5177 max)

    let original_position = position.clone();
    let mut visited_positions: HashSet<IVec2> = HashSet::from([position]);

    while can_move {
        visited_positions.insert(position);
        (can_move, position, direction) = move_guard(position, direction, &map, rows, columns);
    }
    visited_positions.remove(&original_position);

    // loop through and add an obstacle at each visited point and see if we loop
    let mut count = 0;

    for obstacle in visited_positions {
        position = original_position;
        direction = IVec2::new(0, -1);
        can_move = true;
        let mut visited: HashSet<(IVec2, IVec2)> = HashSet::new();
        let mut last_position = position;

        // add obstacle
        match map.get_mut(&obstacle) {
            Some(value) => *value = MapType::Obstacle,
            None => println!("Key does not exist in the HashMap."),
        }

        // loop and increment count if we have a looping candidate
        while can_move {
            if last_position != position && visited.contains(&(position, direction)) {
                count += 1;
                can_move = false;
                // println!("{}", obstacle.as_vec2());
                continue;
            } else {
                visited.insert((position, direction));
            }

            last_position = position;

            (can_move, position, direction) = move_guard(position, direction, &map, rows, columns);
        }

        // remove obstacle
        match map.get_mut(&obstacle) {
            Some(value) => *value = MapType::Open,
            None => println!("Key does not exist in the HashMap."),
        }
    }

    Ok(count.to_string())
}

fn move_guard2(
    position: IVec2,
    direction: IVec2,
    map: &HashMap<IVec2, MapType>,
    rows: i32,
    columns: i32,
) -> (bool, IVec2, IVec2) {
    // get the next position and what is there on the map
    let mut next_position = position + direction.xy();

    // is the guard still on the map?
    if next_position.x < 0
        || next_position.x >= columns
        || next_position.y < 0
        || next_position.y >= rows
    {
        return (false, position, direction);
    }

    // is it an obstacle
    let item = *map
        .get(&IVec2::new(next_position.x, next_position.y))
        .unwrap();

    let next_direction: IVec2;
    if item == MapType::Obstacle {
        next_position = position;
        // change direction
        if direction == IVec2::new(0, -1) {
            next_direction = IVec2::new(1, 0);
        } else if direction == IVec2::new(1, 0) {
            next_direction = IVec2::new(0, 1);
        } else if direction == IVec2::new(0, 1) {
            next_direction = IVec2::new(-1, 0);
        } else if direction == IVec2::new(-1, 0) {
            next_direction = IVec2::new(0, -1);
        } else {
            next_direction = direction;
        }
    } else {
        // otherwise update position
        next_direction = direction;
    }

    (true, next_position, next_direction)
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
/* possabilities are where we visited in the first run, excluding the start.
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XO^XXXX.
.XXXXOOX#.
#OXOXXXX..
......#O..
*/
