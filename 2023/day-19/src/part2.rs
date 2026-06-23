use std::{collections::HashMap};
use std::cmp::Ordering;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, line_ending, multispace1,
    },
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use common::custom_error::AocError;

#[derive(Debug)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>
}

#[derive(Debug, Eq, PartialEq)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Eq, PartialEq)]
enum Target<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

#[derive(Debug, Eq, PartialEq)]
enum Rule<'a> {
    Test {
        category: &'a str,
        condition: Condition,
        value: u32,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

impl<'a> Rule<'a> {
    fn apply_to(&self, part: &Part) -> Option<&Target> {
        match self {
            Rule::Test {
                category,
                condition,
                value,
                target,
            } => {
                let test_value = match *category {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => {
                        unreachable!(
                            "no letters that aren't xmas"
                        );
                    }
                };

                let cond = match condition {
                    Condition::LessThan => Ordering::Less,
                    Condition::GreaterThan => {
                        Ordering::Greater
                    }
                };

                (test_value.cmp(value) == cond)
                    .then_some(target)
            }
            Rule::Target(target) => Some(target),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

fn part(input: &str) -> IResult<&str, Part> {
    delimited(
        complete::char('{'),
        fold_many1(
            terminated(
                separated_pair(
                    alpha1,
                    complete::char('='),
                    complete::u32,
                ),
                opt(tag(","))
            ),
            Part::default,
            |mut part, (next_field, count)| {
                match next_field {
                    "x" => {
                        part.x = count;
                    }
                    "m" => {
                        part.m = count;
                    }
                    "a" => {
                        part.a = count;
                    }
                    "s" => {
                        part.s = count;
                    }
                    _ => unreachable!(
                        "no letters that aren't xmas"
                    )
                }
                part
            },
        ),
        complete::char('}')
    )(input)
}

fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, part)(input)
}

fn target(input: &str) -> IResult<&str, Target> {
    alt((
        tag("A").map(|_| Target::Accepted),
        tag("R").map(|_| Target::Rejected),
        alpha1.map(|workflow_id| {
            Target::Workflow(workflow_id)
        }),
    ))(input)
}

fn rule_test(input: &str) -> IResult<&str, Rule> {
    let (input, category) = alpha1(input)?;
    let (input, condition) = alt((
        complete::char('>').map(|_| Condition::GreaterThan),
        complete::char('<').map(|_| Condition::LessThan),
    ))(input)?;
    let (input, value) = complete::u32(input)?;
    let (input, _) = complete::char(':')(input)?;
    let (input, target) = target(input)?;

    Ok((
        input,
        Rule::Test {
            category,
            condition,
            value,
            target,
        },
    ))
}

fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, id) = alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            alt((rule_test, target.map(Rule::Target))),
        ),
        complete::char('}'),
    )(input)?;

    Ok((input, Workflow { id, rules }))
}

fn workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, workflows) = separated_list1(line_ending, workflow)(input)?;

    Ok((
        input,
        workflows.into_iter().map(|w| (w.id, w)).collect() // implicitly converted to hashmap
    ))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = multispace1(input)?;
    let (input, parts) = parts(input)?;

    Ok((input, (workflows, parts)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (workflows, _)) = parse_input(input).unwrap();

    for workflow in workflows {
        // convert to ranges
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
