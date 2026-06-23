use crate::process_input;
use common::custom_error::{Error, Result};
use common::parsing::Span;
use glam::IVec2;
use std::collections::HashMap;

pub fn process(input: &str) -> Result<String, Error> {
    let columns = input.lines().next().unwrap().len() as i32;
    let rows = input.lines().count() as i32;
    let (_, mut grid) = process_input(Span::new(input)).expect("invalid input");
    let mut count = 0;

    for x in 1..rows - 1 {
        for y in 1..columns - 1 {
            // Skip anywhere the start letter does not match
            match grid.get(&IVec2::new(x, y)) {
                None => {
                    continue;
                }
                Some(ch) => {
                    if ch != &'A' {
                        continue;
                    }
                }
            }

            // Now brut force all directions
            count += check_word(x, y, &mut grid, 0);
            count += check_word(x, y, &mut grid, 1);
            count += check_word(x, y, &mut grid, 2);
            count += check_word(x, y, &mut grid, 3);
            count += check_word(x, y, &mut grid, 4);
            count += check_word(x, y, &mut grid, 5);
            count += check_word(x, y, &mut grid, 6);
            count += check_word(x, y, &mut grid, 7);
        }
    }

    Ok(count.to_string())
}

fn check_word(x: i32, y: i32, grid: &mut HashMap<IVec2, char>, dir: i32) -> i32 {
    let mut x0 = 0;
    let mut y0 = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut x3 = 0;
    let mut y3 = 0;

    // no left right or up down
    // if dir == 0 {
    //     x0 = x - 1; y0 = y + 0;
    //     x1 = x + 1; y1 = y + 0;
    //     x2 = x + 0; y2 = y - 1;
    //     x3 = x + 0; y3 = y + 1;
    // }
    // else if dir == 1 {
    //     x0 = x + 1; y0 = y + 0;
    //     x1 = x - 1; y1 = y + 0;
    //     x2 = x + 0; y2 = y + 1;
    //     x3 = x + 0; y3 = y - 1;
    // }
    // else if dir == 2 {
    //     x0 = x - 1; y0 = y + 0;
    //     x1 = x + 1; y1 = y + 0;
    //     x2 = x + 0; y2 = y + 1;
    //     x3 = x + 0; y3 = y - 1;
    // }
    // else if dir == 3 {
    //     x0 = x + 1; y0 = y + 0;
    //     x1 = x - 1; y1 = y + 0;
    //     x2 = x + 0; y2 = y - 1;
    //     x3 = x + 0; y3 = y + 1;
    // }
    //else
    if dir == 4 {
        x0 = x - 1;
        y0 = y - 1;
        x1 = x + 1;
        y1 = y + 1;
        x2 = x + 1;
        y2 = y - 1;
        x3 = x - 1;
        y3 = y + 1;
    } else if dir == 5 {
        x0 = x - 1;
        y0 = y - 1;
        x1 = x + 1;
        y1 = y + 1;
        x2 = x - 1;
        y2 = y + 1;
        x3 = x + 1;
        y3 = y - 1;
    } else if dir == 6 {
        x0 = x + 1;
        y0 = y - 1;
        x1 = x - 1;
        y1 = y + 1;
        x2 = x + 1;
        y2 = y + 1;
        x3 = x - 1;
        y3 = y - 1;
    } else if dir == 7 {
        x0 = x - 1;
        y0 = y + 1;
        x1 = x + 1;
        y1 = y - 1;
        x2 = x + 1;
        y2 = y + 1;
        x3 = x - 1;
        y3 = y - 1;
    }

    if check_char(x0, y0, grid, 'M')
        && check_char(x1, y1, grid, 'S')
        && check_char(x2, y2, grid, 'M')
        && check_char(x3, y3, grid, 'S')
    {
        return 1;
    }

    0
}

fn check_char(x: i32, y: i32, grid: &mut HashMap<IVec2, char>, c: char) -> bool {
    return match grid.get(&IVec2::new(x, y)) {
        None => false,
        Some(ch) => {
            if ch == &c {
                true
            } else {
                false
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
