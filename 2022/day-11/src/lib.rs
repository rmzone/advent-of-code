use nom::Parser;
use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, multispace1, one_of, space1};
use nom::IResult;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::terminated;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Operation {
    Add(i32),
    Multiply(i32),
}

#[derive(Debug)]
pub struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    test_true: i32,
    test_false: i32,
    inspected_items_count: i32,
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<i32, Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n"), monkey).parse(input)?;
    let monkeys = monkeys
        .into_iter()
        .collect();

    Ok((input, monkeys))
}

fn monkey(input: &str) -> IResult<&str, (i32, Monkey)> {
    let (input, (_, id, _)) = terminated((tag("Monkey "), complete::i32, tag(":")), line_ending).parse(input)?;
    let (input, (_, _, items)) = terminated((space1, tag("Starting items: "), separated_list0(tag(", "), complete::i32)), line_ending).parse(input)?;
    let (input, operation) = terminated(operation, line_ending).parse(input)?;
    let (input, (_, _, test)) = terminated((space1, tag("Test: divisible by "), complete::i32), line_ending).parse(input)?;
    let (input, (_, _, test_true)) = terminated((space1, tag("If true: throw to monkey "), complete::i32), line_ending).parse(input)?;
    let (input, (_, _, test_false)) = terminated((space1, tag("If false: throw to monkey "), complete::i32,), line_ending).parse(input)?;

    Ok((input, (id, Monkey {
        id,
        items,
        operation,
        test,
        test_true,
        test_false,
        inspected_items_count: 0,
    })))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ").parse(input)?;
    let (input, operation) = one_of("+*").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, arg) = alt((
                  digit1,
                  tag("old")
              )).parse(input)?;

    let arg = if arg == "old" { -1 } else { arg.parse::<i32>().unwrap() };

    let operation = match operation {
        '+' => Operation::Add(arg),
        '*' => Operation::Multiply(arg),
        _ => unreachable!(),
    };

    Ok((input, operation))
}
