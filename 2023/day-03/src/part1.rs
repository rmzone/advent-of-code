use common::custom_error::AocError;

fn build_matrix(input: &str) -> (Vec<char>, u32, u32) {
    let mut matrix: Vec<char> = vec![];
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        width = 0;
        height += 1;
        for c in line.chars() {
            matrix.push(c);
            width += 1;
        }
    }

    (matrix, width, height)
}

fn is_valid(
    matrix: &Vec<char>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    width: u32,
) -> bool {
    // if we find any non '.' or number in range then we have a valid part number
    for y in start_y..end_y + 1 as usize {
        for x in start_x..end_x + 1 as usize {
            if let Some(c) = matrix.get(x + y * width as usize) {
                if !c.is_digit(10) && *c != '.' {
                    return true;
                }
            }
        }
    }

    false
}

fn check(
    matrix: &Vec<char>,
    part_numbers: &mut Vec<u32>,
    part_number: &mut String,
    pos_x: usize,
    pos_y: usize,
    width: u32,
    height: u32,
) {
    if part_number.len() > 0 {
        // build box around current selection (remember limits)
        let len = part_number.len() as i32;

        let mut start_x: i32 = pos_x as i32 - 1;
        if start_x < 0 {
            start_x = 0;
        }

        let mut start_y: i32 = pos_y as i32 - 1;
        if start_y < 0 {
            start_y = 0;
        }

        let mut end_x: i32 = pos_x as i32 + len;
        if end_x >= width as i32 {
            end_x = width as i32 - 1;
        }

        let mut end_y: i32 = pos_y as i32 + 1;
        if end_y >= height as i32 {
            end_y = height as i32 - 1;
        }

        if is_valid(
            &matrix,
            start_x as usize,
            start_y as usize,
            end_x as usize,
            end_y as usize,
            width,
        ) {
            if let Ok(a) = part_number.parse::<u32>() {
                part_numbers.push(a);
            }
        }
    }

    // reset
    part_number.clear();
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut part_numbers: Vec<u32> = vec![];
    let (matrix, width, height) = build_matrix(input);

    let mut part_number = "".to_string();
    let mut last_row = 0;

    for y in 0..height as usize {
        for x in 0..width as usize {
            if last_row != y {
                let len = &part_number.len();
                let start_x: usize = width as usize - len;
                let start_y: usize = y - 1;
                check(
                    &matrix,
                    &mut part_numbers,
                    &mut part_number,
                    start_x,
                    start_y,
                    width,
                    height,
                );
                last_row = y;
            }

            if let Some(c) = matrix.get(x + y * height as usize) {
                if c.is_digit(10) {
                    part_number.push(*c);
                    continue;
                }
            }

            let len = &part_number.len();
            check(
                &matrix,
                &mut part_numbers,
                &mut part_number,
                x - len,
                y,
                width,
                height,
            );
        }
    }

    // sum list
    let sum = part_numbers.iter().sum::<u32>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
