use crate::part1::Square::NorthSouth;
use common::custom_error::AocError;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Square {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Node {
    north: Option<UVec2>,
    south: Option<UVec2>,
    east: Option<UVec2>,
    west: Option<UVec2>,
}

impl Node {
    pub fn new(n: Option<UVec2>, s: Option<UVec2>, e: Option<UVec2>, w: Option<UVec2>) -> Self {
        Self {
            north: n,
            south: s,
            east: e,
            west: w,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct UVec2 {
    x: u32,
    y: u32,
}

fn determine_start_type(map: &BTreeMap<UVec2, Square>, start: UVec2) -> Square {
    let n = map
        .get(&UVec2 {
            x: start.x,
            y: start.y - 1,
        })
        .unwrap_or(&Square::Ground);
    let s = map
        .get(&UVec2 {
            x: start.x,
            y: start.y + 1,
        })
        .unwrap_or(&Square::Ground);
    let e = map
        .get(&UVec2 {
            x: start.x + 1,
            y: start.y,
        })
        .unwrap_or(&Square::Ground);
    let w = map
        .get(&UVec2 {
            x: start.x - 1,
            y: start.y,
        })
        .unwrap_or(&Square::Ground);

    let n_valid = n == &Square::NorthSouth || n == &Square::SouthEast || n == &Square::SouthWest;
    let s_valid = s == &Square::NorthSouth || s == &Square::NorthEast || s == &Square::NorthWest;
    let e_valid = e == &Square::EastWest || e == &Square::SouthEast || e == &Square::NorthEast;
    let w_valid = w == &Square::EastWest || w == &Square::NorthWest || w == &Square::SouthWest;

    if n_valid && s_valid {
        Square::NorthSouth
    } else if e_valid && w_valid {
        Square::EastWest
    } else if e_valid && s_valid {
        Square::SouthEast
    } else if w_valid && s_valid {
        Square::SouthWest
    } else if n_valid && e_valid {
        Square::NorthEast
    } else if n_valid && w_valid {
        Square::NorthWest
    } else {
        panic!("should not get here!")
    }
}

fn next(current: UVec2, stype: Square, last: UVec2) -> UVec2 {
    let mut a: UVec2 = current;
    let mut b: UVec2 = current;

    match stype {
        Square::NorthSouth => {
            a.y -= 1;
            b.y += 1;
        }
        Square::EastWest => {
            a.x -= 1;
            b.x += 1;
        }
        Square::NorthEast => {
            a.y -= 1;
            b.x += 1;
        }
        Square::NorthWest => {
            a.y -= 1;
            b.x -= 1;
        }
        Square::SouthWest => {
            a.y += 1;
            b.x -= 1;
        }
        Square::SouthEast => {
            a.y += 1;
            b.x += 1;
        }
        _ => {
            panic!("should not get here")
        }
    }

    // dbg!(stype, &current, &a, &b);

    if a == last {
        b
    } else {
        a
    }
}

fn build_map(input: &str) -> (BTreeMap<UVec2, Square>, u32, u32, UVec2) {
    use Square::*;
    let mut map: BTreeMap<UVec2, Square> = BTreeMap::new();
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut start: UVec2 = UVec2 { x: 0, y: 0 };

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        x = 0;
        for c in line.chars() {
            let s = match c {
                '|' => NorthSouth,
                '-' => EastWest,
                'L' => NorthEast,
                'J' => NorthWest,
                '7' => SouthWest,
                'F' => SouthEast,
                '.' => Ground,
                'S' => {
                    start = UVec2 { x, y };
                    Start
                }
                _ => panic!("should never get here!"),
            };

            // ignore ground
            if s != Ground {
                map.insert(UVec2 { x, y }, s);
            }
            x += 1;
        }
        y += 1;
    }

    // dbg!(&map, &x, &y);

    (map, x, y, start)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    use Square::*;
    let (map, width, height, start) = build_map(input);
    let mut last = UVec2 { x: 999, y: 999 };
    let mut current = start;
    let mut distance = 0;

    // temp code
    let mut me = determine_start_type(&map, start);
    // dbg!(&current, me);

    while me != Square::Start {
        let temp = next(current, me, last);
        last = current;
        current = temp;
        me = *map.get(&current).expect("valid square");
        distance += 1;
    }

    distance /= 2;

    Ok(distance.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        "4"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
