pub mod part1;
pub mod part2;

use glam::{IVec2, Vec2Swizzles};
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MapType {
    Open,
    Guard,
    Visited,
    Obstacle,
}

pub fn move_guard(
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

pub fn process_input(input: &str) -> HashMap<IVec2, MapType> {
    let mut map: HashMap<IVec2, MapType> = HashMap::new();
    let mut row = 0;

    for line in input.lines() {
        let mut column = 0;
        for c in line.chars() {
            match c {
                '.' => map.insert(IVec2::new(column, row), MapType::Open),
                '^' => map.insert(IVec2::new(column, row), MapType::Guard),
                '#' => map.insert(IVec2::new(column, row), MapType::Obstacle),
                'X' => map.insert(IVec2::new(column, row), MapType::Visited),
                _ => panic!(),
            };

            column += 1;
        }

        row += 1;
    }

    map
}
