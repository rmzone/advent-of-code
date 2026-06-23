use nom::bytes::complete::{tag, take};
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq)]
pub struct Group {
    garbage: Vec<String>,
    children: Vec<Group>,
}

pub fn parse_input(input: &str) -> IResult<&str, Group> {
    let (input, _) = tag("{").parse(input)?;
    let (input, g) = opt(parse_garbage).parse(input)?;
    let (input, c) = opt(parse_groups).parse(input)?;
    let (input, _) = tag("}").parse(input)?;

    let garbage = g.unwrap_or_default();
    let children = c.unwrap_or_default();

    Ok((input, Group { garbage, children }))
}

pub fn parse_groups(input: &str) -> IResult<&str, Vec<Group>> {
    separated_list0(tag(","), parse_group).parse(input)
}

pub fn parse_group(input: &str) -> IResult<&str, Group> {
    let (input, _) = tag("{").parse(input)?;
    let (input, g) = opt(parse_garbage).parse(input)?;
    let (input, c) = opt(parse_groups).parse(input)?;
    let (input, _) = tag("}").parse(input)?;

    let garbage = g.unwrap_or_default();
    let children = c.unwrap_or_default();

    Ok((input, Group { garbage, children }))
}

pub fn parse_garbage(input: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(","), delimited(tag("<"), parse_inner_garbage, tag(">"))).parse(input)
}

pub fn parse_inner_garbage(input: &str) -> IResult<&str, String> {
    let mut output = "".to_string();
    let mut input = input;

    loop {
        let ch = opt(one_of("!<abcdefghijklmnopqrstuvwzyz,\"'*{}")).parse(input)?;
        if let (i, Some(ch)) = ch {
            input = i;
            match ch {
                '!' => {
                    let (i, _) = take(1usize).parse(input)?;
                    input = i;
                    continue;
                }
                '<' | '{' | '}' | ',' | '"' => continue,
                _ => {
                    output.push(ch);
                }
            }
        } else {
            break;
        }
    }

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("{}"),
            Ok((
                "",
                Group {
                    garbage: vec![],
                    children: vec![]
                }
            )),
            "test_parse_input_test1"
        );

        assert_eq!(
            parse_input("{{{}}}"),
            Ok((
                "",
                Group {
                    garbage: vec![],
                    children: vec![Group {
                        garbage: vec![],
                        children: vec![Group {
                            garbage: vec![],
                            children: vec![]
                        }]
                    }]
                }
            )),
            "test_parse_input_test2"
        );

        assert_eq!(
            parse_input("{{},{}}"),
            Ok((
                "",
                Group {
                    garbage: vec![],
                    children: vec![
                        Group {
                            garbage: vec![],
                            children: vec![]
                        },
                        Group {
                            garbage: vec![],
                            children: vec![]
                        }
                    ]
                }
            )),
            "test_parse_input_test3"
        );

        assert_eq!(
            parse_input("{{{},{},{{}}}}"),
            Ok((
                "",
                Group {
                    garbage: vec![],
                    children: vec![Group {
                        garbage: vec![],
                        children: vec![
                            Group {
                                garbage: vec![],
                                children: vec![]
                            },
                            Group {
                                garbage: vec![],
                                children: vec![]
                            },
                            Group {
                                garbage: vec![],
                                children: vec![Group {
                                    garbage: vec![],
                                    children: vec![]
                                }]
                            }
                        ]
                    }]
                }
            )),
            "test_parse_input_test4"
        );

        assert_eq!(
            parse_input("{<{},{},{{}}>}"),
            Ok((
                "",
                Group {
                    garbage: vec!["".to_string()],
                    children: vec![]
                }
            )),
            "test_parse_input_test5"
        );

        assert_eq!(
            parse_input("{<a>,<a>,<a>,<a>}"),
            Ok((
                "",
                Group {
                    garbage: vec![
                        "a".to_string(),
                        "a".to_string(),
                        "a".to_string(),
                        "a".to_string()
                    ],
                    children: vec![]
                }
            )),
            "test_parse_input_test6"
        );

        assert_eq!(
            parse_input("{{<a>},{<a>},{<a>},{<a>}}"),
            Ok((
                "",
                Group {
                    garbage: vec![],
                    children: vec![
                        Group {
                            garbage: vec!["a".to_string()],
                            children: vec![]
                        },
                        Group {
                            garbage: vec!["a".to_string()],
                            children: vec![]
                        },
                        Group {
                            garbage: vec!["a".to_string()],
                            children: vec![]
                        },
                        Group {
                            garbage: vec!["a".to_string()],
                            children: vec![]
                        }
                    ]
                }
            )),
            "test_parse_input_test7"
        );

        assert_eq!(
            parse_input("{{<!>},{<!>},{<!>},{<a>}}"),
            Ok((
                "",
                Group {
                    garbage: vec![],
                    children: vec![Group {
                        garbage: vec!["a".to_string()],
                        children: vec![]
                    }]
                }
            )),
            "test_parse_input_test8"
        );
    }

    // Here are some self-contained pieces of garbage:
    // <>, empty garbage.
    // <random characters>, garbage containing random characters.
    // <<<<>, because the extra < are ignored.
    // <{!>}>, because the first > is canceled.
    // <!!>, because the second ! is canceled, allowing the > to terminate the garbage.
    // <!!!>>, because the second ! and the first > are canceled.
    // <{o"i!a,<{i<a>, which ends at the first >.
    #[test]
    fn test_parse_garbage() {
        assert_eq!(parse_garbage("<>"), Ok(("", vec!["".to_string()])), "test1");
        assert_eq!(
            parse_garbage("<abcd>"),
            Ok(("", vec!["abcd".to_string()])),
            "test2"
        );
        assert_eq!(
            parse_garbage("<<<<>"),
            Ok(("", vec!["".to_string()])),
            "test3"
        );
        assert_eq!(
            parse_garbage("<{!>}>"),
            Ok(("", vec!["".to_string()])),
            "test4"
        );
        assert_eq!(
            parse_garbage("<!!>"),
            Ok(("", vec!["".to_string()])),
            "test5"
        );
        assert_eq!(
            parse_garbage("<!!!>>"),
            Ok(("", vec!["".to_string()])),
            "test6"
        );
        assert_eq!(
            parse_garbage("<{o\"i!a,<{i<a>"),
            Ok(("", vec!["oiia".to_string()])),
            "test7"
        );
        assert_eq!(
            parse_garbage("<{},{},{{}}>"),
            Ok(("", vec!["".to_string()])),
            "test8"
        );
        assert_eq!(
            parse_garbage("<a>,<a>,<a>,<a>"),
            Ok((
                "",
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string()
                ]
            )),
            "test9"
        );
    }
}
