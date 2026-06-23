use common::parsing::{token, Span};
use glam::IVec2;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn process_input(input: Span) -> IResult<Span, HashMap<char, Vec<IVec2>>> {
    let valid_tokens = ".0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let (input, cells) = separated_list1(line_ending, many1(token(valid_tokens)))(input)?;

    let map = cells
        .into_iter()
        .flatten()
        .fold(HashMap::new(), |mut acc: HashMap<char, Vec<IVec2>>, cell| {
            if cell.1 != '.' {
                acc.entry(cell.1).or_default().push(cell.0);
            }
            acc
        });

    Ok((input, map))
}
