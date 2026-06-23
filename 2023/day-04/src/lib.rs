use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{IResult, Parser};
use std::collections::HashSet;

pub mod part1;
pub mod part2;

struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        match self.num_matches().checked_sub(1) {
            Some(num) => 2u32.pow(num as u32),
            None => 0,
        }
    }

    fn num_matches(&self) -> usize {
        self.winning_numbers.intersection(&self.my_numbers).count()
    }
}

fn parse_set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    // consume and discard the first part of each line.
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;

    // parse the rest into two sets.
    separated_pair(parse_set, tuple((tag("|"), space1)), parse_set)
        .map(|(winning_numbers, my_numbers)| Card {
            winning_numbers,
            my_numbers,
        })
        .parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)
}
