use nom::bytes::complete::{is_a, tag};
use nom::Parser;
use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use array2d::Array2D;


pub mod part1;
pub mod part2;

pub const START_PATTERN: &str = ".#./..#/###";

#[derive(Debug)]
pub struct Rule<'a> {
    size: usize,
    pattern: &'a str,
    enhancement: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(pattern: &'a str, enhancement: &'a str) -> Self {
        Rule {
            size: pattern.split('/').count(),
            pattern,
            enhancement,
        }
    }
}

pub fn to_array(image: &str) -> Array2D<char> {
    let rows: Vec<String> = image.split('/').map(|u| u.to_string()).collect();
    let current_size = rows.iter().count();
    let mut array = Array2D::filled_with('.', current_size, current_size);
    for y in 0..current_size {
        let row = &rows[y];
        for x in 0..current_size {
            array[(y, x)] = row.chars().nth(x).unwrap();
        }
    }

    array
}

pub fn to_string(image_array: &Array2D<char>) -> String {
    let i = image_array.as_rows().iter().fold(
        Vec::new(),
        |mut acc, row| {
            let temp: String = row.iter().map(|a| *a).collect();
            acc.push(temp);
            acc
        }
    ).join("/");

    i
}

pub fn chunk(image: &Array2D<char>, rules: &Vec<Rule>) -> Array2D<char> {
    let current_size = image.column_len();
    let mut chunk_size = 1;
    if current_size % 2 == 0 {
        chunk_size = current_size / 2;
    }
    else if current_size % 3 == 0 {
        chunk_size = current_size / 3;
    }

    // build empty holder for new image
    let new_size = current_size / chunk_size; // size of each piece
    let mut inner = Array2D::filled_with('.', current_size, current_size);
    let mut output = Array2D::filled_with('.', chunk_size, chunk_size);

    // perform the split
    for ((row, column), elem) in image.enumerate_row_major() {

    }

    todo!()
}

pub fn merge_chunks(chunks: Vec<String>) -> String {
    todo!()
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(line_ending, rule).parse(input)
}

fn rule(input: &str) -> IResult<&str, Rule<'_>> {
    let (input, (pattern, enhancement)) = separated_pair(is_a(".#/"), tag(" => "), is_a(".#/")).parse(input)?;
    Ok((input, Rule::new(pattern, enhancement)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_convert() {
        let input = ".#./..#/###";
        let as_array = to_array(&input);
        let as_string = to_string(&as_array);
        assert_eq!(input, as_string);
    }
}
