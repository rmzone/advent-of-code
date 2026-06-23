pub mod part1;
pub mod part2;

use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    fold_many1(
        terminated(parse_line, opt(line_ending)),
        || (Vec::new(), Vec::new()),
        |mut acc: (Vec<i32>, Vec<i32>), (l, r)| {
            acc.0.push(l);
            acc.1.push(r);
            acc
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, space1, complete::i32)(input)
}
