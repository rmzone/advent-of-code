use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete;
use nom::character::complete::{alpha1, alphanumeric1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::collections::BTreeMap;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Node<'a> {
    File(&'a str, usize),
    Directory(&'a str),
}

pub fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, usize>),
    directive: &'a Directive,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, usize>) {
    match directive {
        Directive::ChangeDirectory(CdType::Root) => {
            context.push("/");
        }
        Directive::ChangeDirectory(CdType::Up) => {
            context.pop();
        }
        Directive::ChangeDirectory(CdType::Down(name)) => {
            context.push(name);
        }
        Directive::List(nodes) => {
            // get the sizes of all the files in the current directory
            let sum = nodes
                .iter()
                .filter_map(|file| {
                    if let Node::File(.., size) = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<usize>();

            // update all the sizes for this context all the way back to the root
            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
    }

    (context, sizes)
}

#[derive(Debug)]
pub enum CdType<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
pub enum Directive<'a> {
    ChangeDirectory(CdType<'a>),
    List(Vec<Node<'a>>),
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Directive<'_>>> {
    separated_list1(line_ending, alt((change_directory, list))).parse(input)
}

fn change_directory(input: &str) -> IResult<&str, Directive<'_>> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/"))).parse(input)?;
    let op = match dir {
        "/" => Directive::ChangeDirectory(CdType::Root),
        ".." => Directive::ChangeDirectory(CdType::Up),
        name => Directive::ChangeDirectory(CdType::Down(name)),
    };

    Ok((input, op))
}

fn list(input: &str) -> IResult<&str, Directive<'_>> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, files) = separated_list1(line_ending, alt((file, directory))).parse(input)?;

    Ok((input, Directive::List(files)))
}

fn directory(input: &str) -> IResult<&str, Node<'_>> {
    map((tag("dir "), alphanumeric1), |(_, name)| {
        Node::Directory(name)
    })
    .parse(input)
}

fn file(input: &str) -> IResult<&str, Node<'_>> {
    map(
        separated_pair(
            complete::usize,
            tag(" "),
            is_a("qwertyuiopasdfghjklzxcvbnm."),
        ),
        |(size, name)| Node::File(name, size),
    )
    .parse(input)
}
