use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::cmp::Ordering;

pub mod part1;
pub mod part2;

pub type UpdatesSet = Vec<u32>;

#[derive(Debug)]
pub struct Order {
    pub first: u32,
    pub second: u32,
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Order>, Vec<UpdatesSet>)> {
    let (input, rules) = terminated(parse_rules, line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (_, updates) = parse_updates(input)?;

    Ok((input, (rules, updates)))
}

pub fn is_valid(update: &UpdatesSet, orders: &Vec<Order>) -> bool {
    for i in 1..update.len() {
        // start at index 1 since there is nothing before this
        let current = update[i];
        let slice = &update[..i];

        for order in orders {
            if order.first == current {
                match slice.iter().find(|&a| a == &order.second) {
                    None => {}
                    Some(_) => return false,
                }
            }
        }
    }

    true
}

pub fn find_middle(rules: &UpdatesSet) -> u32 {
    let index = rules.len() / 2;
    rules[index]
}

pub fn fix(update: &UpdatesSet, rules: &Vec<Order>) -> UpdatesSet {
    let mut fixed = update.clone();

    fixed.sort_by(|a, b| {
        match rules
            .iter()
            .find(|&rule| rule.first == *a && rule.second == *b)
        {
            None => Ordering::Greater,
            Some(_) => Ordering::Less,
        }
    });

    fixed
}

fn parse_rule(input: &str) -> IResult<&str, Order> {
    let (input, (first, second)) = separated_pair(complete::u32, tag("|"), complete::u32)(input)?;
    Ok((input, Order { first, second }))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Order>> {
    separated_list1(line_ending, parse_rule)(input)
}

fn parse_update(input: &str) -> IResult<&str, UpdatesSet> {
    let (input, items) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((input, items))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<UpdatesSet>> {
    separated_list1(line_ending, parse_update)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_middle() {
        let vec: UpdatesSet = vec![1, 2, 3, 4, 5];

        assert_eq!(find_middle(&vec), 3);
    }
}
