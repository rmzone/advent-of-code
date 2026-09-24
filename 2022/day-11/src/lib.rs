use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, one_of, space1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::terminated;
use std::collections::{HashMap, VecDeque};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    test_true: i32,
    test_false: i32,
    inspected_items_count: i32,
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<i32, Monkey>> {
    let (input, monkeys) = separated_list1(line_ending, monkey).parse(input)?;
    let monkeys = monkeys.into_iter().collect();

    Ok((input, monkeys))
}

fn monkey(input: &str) -> IResult<&str, (i32, Monkey)> {
    let (input, (_, id, _)) =
        terminated((tag("Monkey "), complete::i32, tag(":")), line_ending).parse(input)?;
    let (input, (_, _, items)) = terminated(
        (
            space1,
            tag("Starting items: "),
            separated_list0(tag(", "), complete::u64),
        ),
        line_ending,
    )
    .parse(input)?;
    let (input, operation) = terminated(operation, line_ending).parse(input)?;
    let (input, (_, _, test)) = terminated(
        (space1, tag("Test: divisible by "), complete::u64),
        line_ending,
    )
    .parse(input)?;
    let (input, (_, _, test_true)) = terminated(
        (space1, tag("If true: throw to monkey "), complete::i32),
        line_ending,
    )
    .parse(input)?;
    let (input, (_, _, test_false)) = terminated(
        (space1, tag("If false: throw to monkey "), complete::i32),
        line_ending,
    )
    .parse(input)?;

    let items = VecDeque::from(items);

    Ok((
        input,
        (
            id,
            Monkey {
                items,
                operation,
                test,
                test_true,
                test_false,
                inspected_items_count: 0,
            },
        ),
    ))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ").parse(input)?;
    let (input, operation) = one_of("+*").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, arg) = alt((digit1, tag("old"))).parse(input)?;

    let arg = if arg == "old" {
        0
    } else {
        arg.parse::<u64>().unwrap()
    };

    let operation = match operation {
        '+' => Operation::Add(arg),
        '*' => Operation::Multiply(arg),
        _ => unreachable!(),
    };

    Ok((input, operation))
}
