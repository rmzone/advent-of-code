use glam::IVec2;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MapType {
    Open,
    ToiletPaper,
}

pub fn process_input(input: &str) -> HashMap<IVec2, MapType> {
    let mut map: HashMap<IVec2, MapType> = HashMap::new();
    let mut row = 0;

    for line in input.lines() {
        let mut column = 0;
        for c in line.chars() {
            match c {
                '.' => map.insert(IVec2::new(column, row), MapType::Open),
                '@' => map.insert(IVec2::new(column, row), MapType::ToiletPaper),
                _ => panic!(),
            };

            column += 1;
        }

        row += 1;
    }

    map
}

pub fn accessible(location: &IVec2, map: &HashMap<IVec2, MapType>) -> Vec<IVec2> {
    let mut result = Vec::new();

    let north = *location + IVec2::new(0, -1);
    if map.contains_key(&north) && map[&north] == MapType::ToiletPaper {
        result.push(north);
    }

    let south = *location + IVec2::new(0, 1);
    if map.contains_key(&south) && map[&south] == MapType::ToiletPaper {
        result.push(south);
    }

    let east = *location + IVec2::new(1, 0);
    if map.contains_key(&east) && map[&east] == MapType::ToiletPaper {
        result.push(east);
    }

    let west = *location + IVec2::new(-1, 0);
    if map.contains_key(&west) && map[&west] == MapType::ToiletPaper {
        result.push(west);
    }

    let north_east = *location + IVec2::new(1, -1);
    if map.contains_key(&north_east) && map[&north_east] == MapType::ToiletPaper {
        result.push(north_east);
    }

    let north_west = *location + IVec2::new(-1, -1);
    if map.contains_key(&north_west) && map[&north_west] == MapType::ToiletPaper {
        result.push(north_west);
    }

    let south_east = *location + IVec2::new(1, 1);
    if map.contains_key(&south_east) && map[&south_east] == MapType::ToiletPaper {
        result.push(south_east);
    }

    let south_west = *location + IVec2::new(-1, 1);
    if map.contains_key(&south_west) && map[&south_west] == MapType::ToiletPaper {
        result.push(south_west);
    }

    result
}
