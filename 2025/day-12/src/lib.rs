use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::{fold_many1, many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

pub mod part1;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Shape {
    pub shape: Vec<Vec<bool>>,
}

impl Shape {
    pub fn size(&self) -> usize {
        self.shape.iter().flatten().filter(|&&v| v).count()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    pub shapes: Vec<usize>,
}

pub fn parse_input(input: &str) -> IResult<&str, (HashMap<usize, Shape>, Vec<Region>)> {
    let (input, shapes) = shapes(input)?;
    let (input, regions) = regions(input)?;

    Ok((input, (shapes, regions)))
}

fn shapes(input: &str) -> IResult<&str, HashMap<usize, Shape>> {
    let (input, items) = many1(shape).parse(input)?;
    let shapes = items.into_iter().collect();

    Ok((input, shapes))
}

fn shape(input: &str) -> IResult<&str, (usize, Shape)> {
    let (input, (index, _)) = terminated((complete::usize, tag(":")), line_ending).parse(input)?;
    let (input, shape) = many1(terminated(row, line_ending)).parse(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, (index, Shape { shape })))
}

fn row(input: &str) -> IResult<&str, Vec<bool>> {
    fold_many1(
        alt((complete::char('.'), complete::char('#'))),
        || Vec::new(),
        |mut acc: Vec<bool>, item| {
            let value = match item {
                '.' => false,
                '#' => true,
                _ => {
                    panic!("invalid!");
                }
            };
            acc.push(value);
            acc
        },
    ).parse(input)
}

fn regions(input: &str) -> IResult<&str, Vec<Region>> {
    separated_list1(line_ending, region).parse(input)
}

fn region(input: &str) -> IResult<&str, Region> {
    let (input, ((width, height), shapes)) =
        separated_pair(
            separated_pair(
                complete::usize,
                tag("x"),
                complete::usize),
            (tag(":"), space1),
            separated_list1(space1, complete::usize),
        ).parse(input)?;

    Ok((input, Region { width, height, shapes }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_region() {
        let (_, region) = region("12x5: 1 0 1 0 3 2").expect("cannot parse region");

        assert_eq!(
            region,
            Region {
                width: 12,
                height: 5,
                shapes: vec![1, 0, 1, 0, 3, 2]
            }
        );
    }
}
