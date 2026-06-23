use common::custom_error::AocError;
use std::collections::BTreeMap;
use crate::part2::Square::{EastWest, Ground, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest, Start};

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
    north: Option<IVec2>,
    south: Option<IVec2>,
    east: Option<IVec2>,
    west: Option<IVec2>,
}

impl Node {
    pub fn new(n: Option<IVec2>, s: Option<IVec2>, e: Option<IVec2>, w: Option<IVec2>) -> Self {
        Self {
            north: n,
            south: s,
            east: e,
            west: w,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct IVec2 {
    x: i32,
    y: i32,
}

fn determine_start_type(map: &BTreeMap<IVec2, Square>, start: IVec2) -> Square {
    let n = map
        .get(&IVec2 {
            x: start.x,
            y: start.y - 1,
        })
        .unwrap_or(&Square::Ground);
    let s = map
        .get(&IVec2 {
            x: start.x,
            y: start.y + 1,
        })
        .unwrap_or(&Square::Ground);
    let e = map
        .get(&IVec2 {
            x: start.x + 1,
            y: start.y,
        })
        .unwrap_or(&Square::Ground);
    let w = map
        .get(&IVec2 {
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

fn next(current: IVec2, stype: Square, last: IVec2) -> IVec2 {
    let mut a: IVec2 = current;
    let mut b: IVec2 = current;

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

fn build_map(input: &str) -> (BTreeMap<IVec2, Square>, i32, i32, IVec2) {
    use Square::*;
    let mut map: BTreeMap<IVec2, Square> = BTreeMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut start: IVec2 = IVec2 { x: 0, y: 0 };

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
                    start = IVec2 { x, y };
                    Start
                }
                _ => panic!("should never get here!"),
            };
            map.insert(IVec2 { x, y }, s);
            x += 1;
        }
        y += 1;
    }

    // dbg!(&map, &x, &y);

    (map, x, y, start)
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    In,
    Out,
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    use Square::*;
    let (map, width, height, start) = build_map(input);

    // find all parts of the pipe
    let mut pipes: Vec<IVec2> = vec![];
    let mut last = IVec2 { x: 999, y: 999 };
    let mut current = start;
    let mut me = determine_start_type(&map, start);

    // figure out the pipe loop
    while me != Square::Start {
        pipes.push(current);
        let temp = next(current, me, last);
        last = current;
        current = temp;
        me = *map.get(&current).expect("valid square");
    }

    // find enclosed entries
    let mut score = 0;
    for y in 0..height {
        let mut status = Status::Out;

        for x in 0..width {
            let loc = IVec2 { x, y };
            let is_pipe = pipes.contains(&loc);
            let current = map.get(&loc).expect("valid");

            if is_pipe {
                if [NorthSouth, Start, SouthWest, SouthEast].contains(current) {
                    status = match status {
                        Status::In => Status::Out,
                        Status::Out => Status::In,
                    };
                }
            }
            else {
                match status {
                    Status::In => { score += 1; }
                    Status::Out => {}
                }
            }

            // debug map
            if is_pipe {
                match current {
                    NorthSouth => print!("|"),
                    EastWest => print!("-"),
                    NorthEast => print!("L"),
                    NorthWest => print!("J"),
                    SouthWest => print!("7"),
                    SouthEast => print!("F"),
                    Start => print!("S"),
                    Ground => print!(".")
                }
            }
            else {
                match status {
                    Status::In => print!("I"),
                    Status::Out => print!("O")
                }
            }
        }

        print!("   {}\n", &score);
    }

    println!();

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        "4"
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        "10"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
