use common::square_grid::SquareGrid;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub const START_PATTERN: &str = ".#./..#/###";

#[derive(Debug)]
pub struct Rule<'a> {
    pattern: &'a str,
    enhancement: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(pattern: &'a str, enhancement: &'a str) -> Self {
        Rule {
            pattern,
            enhancement,
        }
    }
}

pub fn to_square_grid(image: &str) -> SquareGrid<char> {
    let rows: Vec<String> = image.split('/').map(|u| u.to_string()).collect();
    let current_size = rows.iter().count();
    let mut array = SquareGrid::new(current_size);
    for y in 0..current_size {
        let row = &rows[y];
        for x in 0..current_size {
            array[(x, y)] = row.chars().nth(x).unwrap();
        }
    }

    array
}

pub fn from_square_grid(image_array: &SquareGrid<char>) -> String {
    let i = image_array.iter_rows().fold(
        Vec::new(),
        |mut acc, row| {
            let temp: String = row.iter().map(|a| *a).collect();
            acc.push(temp);
            acc
        },
    ).join("/");

    i
}

pub fn enhance(image: &SquareGrid<char>, rules: &HashMap<SquareGrid<char>, SquareGrid<char>>) -> SquareGrid<char> {
    let current_size = image.size();
    let chunk_size = if current_size.is_multiple_of(2) {
        2
    } else if current_size.is_multiple_of(3) {
        3
    } else {
        panic!();
    };

    let new_size = (current_size / chunk_size) * (chunk_size + 1);
    let mut enhanced_grid = SquareGrid::new(new_size);

    for y in 0..(current_size / chunk_size) {
        for x in 0..(current_size / chunk_size) {
            let subsquare = image.get_square(chunk_size * x, chunk_size * y, chunk_size);
            let enhancement = &rules[&subsquare];
            enhanced_grid.put_square(x * (chunk_size + 1), y * (chunk_size + 1), enhancement);
        }
    }

    enhanced_grid
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<SquareGrid<char>, SquareGrid<char>>> {
    let (input, raw_rules) = separated_list1(line_ending, rule).parse(input)?;
    let mut rules: HashMap<SquareGrid<char>, SquareGrid<char>> = HashMap::new();
    for rule in raw_rules.iter() {
        let src = to_square_grid(rule.pattern);
        let dst = to_square_grid(rule.enhancement);
        for s in src.iter_combos() {
            rules.insert(s, dst.clone());
        }
    }

    Ok((input, rules))
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
        let as_array = to_square_grid(&input);
        let as_string = from_square_grid(&as_array);
        assert_eq!(input, as_string);
    }
}
