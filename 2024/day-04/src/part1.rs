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

    for x in 0..rows {
        for y in 0..columns {
            // Skip anywhere the start letter does not match
            match grid.get(&IVec2::new(x, y)) {
                None => {
                    continue;
                }
                Some(ch) => {
                    if ch != &'X' {
                        continue;
                    }
                }
            }

            // Now brut force all directions
            count += check_word(x, y, &mut grid, 0, -1, rows);
            count += check_word(x, y, &mut grid, 0, 1, rows);
            count += check_word(x, y, &mut grid, -1, 0, rows);
            count += check_word(x, y, &mut grid, 1, 0, rows);
            count += check_word(x, y, &mut grid, 1, 1, rows);
            count += check_word(x, y, &mut grid, 1, -1, rows);
            count += check_word(x, y, &mut grid, -1, 1, rows);
            count += check_word(x, y, &mut grid, -1, -1, rows);
        }
    }

    Ok(count.to_string())
}

fn check_word(
    x: i32,
    y: i32,
    grid: &mut HashMap<IVec2, char>,
    dir_x: i32,
    dir_y: i32,
    size: i32,
) -> i32 {
    let word = ['X', 'M', 'A', 'S'];
    let mut xs = x;
    let mut ys = y;

    for i in 0..word.len() {
        if xs < 0 || xs >= size || ys < 0 || ys >= size {
            return 0;
        }

        match grid.get(&IVec2::new(xs, ys)) {
            None => {
                return 0;
            }
            Some(ch) => {
                if ch != &word[i] {
                    return 0;
                }
            }
        }

        xs += dir_x;
        ys += dir_y;
    }

    // println!("({},{}) {},{}", x, y, dir_x, dir_y);
    1
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
