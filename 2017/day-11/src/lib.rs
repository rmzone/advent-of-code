use glam::IVec3;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

//   \ n  /
// nw +--+ ne
//   /    \
// -+      +-
//   \    /
// sw +--+ se
//   / s  \
//
// 0,0  2,0  4,0 ...
//   1.0  3.0  5.0 ...
// 0,1  2,1  4,1 ...
//   1,1 ...
// 0,2  2,2  4,2
//
// flat top
//  using cube coordinates https://www.redblobgames.com/grids/hexagons/
//      -z
// +y  \ n  /  +x
//   nw +--+ ne
//     /    \
//   -+      +-
//     \    /
//   sw +--+ se
// -x  / s  \  -y
//       +z
// note: q + r + s = 0 always

pub fn move_player(position: IVec3, direction: Direction) -> IVec3 {
    let offset = match direction {
        Direction::North => IVec3::new(0, 1, -1),
        Direction::NorthEast => IVec3::new(1, 0, -1),
        Direction::NorthWest => IVec3::new(-1, 1, 0),
        Direction::South => IVec3::new(0, -1, 1),
        Direction::SouthEast => IVec3::new(1, -1, 0),
        Direction::SouthWest => IVec3::new(-1, 0, 1),
    };

    position + offset
}

// calculate Manhattan distance
// https://www.geeksforgeeks.org/data-science/manhattan-distance/
// https://backdrifting.net/post/064_hex_grids
// (| x1 - x2 | + | y1 - y2 | + | z1 - z2 |) / 2
pub fn distance(position_a: IVec3, position_b: IVec3) -> i32 {
    let dx = (position_a.x - position_b.x).abs();
    let dy = (position_a.y - position_b.y).abs();
    let dz = (position_a.z - position_b.z).abs();

    ((dx + dy + dz) / 2)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list0(tag(","), parse_direction).parse(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, raw) = alpha1.parse(input)?;
    let direction = match raw {
        "n" => Direction::North,
        "ne" => Direction::NorthEast,
        "nw" => Direction::NorthWest,
        "s" => Direction::South,
        "se" => Direction::SouthEast,
        "sw" => Direction::SouthWest,
        _ => panic!("Unknown direction: {}", raw),
    };

    Ok((input, direction))
}
