use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq)]
pub struct Equation {
    test_value: u64,
    operators: Vec<u64>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, parse_equation)(input as &str)
}

fn parse_operators(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (test_value, operators)) =
        separated_pair(complete::u64, tag(": "), parse_operators)(input)?;

    Ok((
        input,
        Equation {
            test_value,
            operators,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operators() {
        assert_eq!(parse_operators("81 40 27"), Ok(("", vec![81, 40, 27])));
    }

    #[test]
    fn test_parse_equation() {
        assert_eq!(
            parse_equation("3267: 81 40 27"),
            Ok((
                "",
                Equation {
                    test_value: 3267,
                    operators: vec![81, 40, 27]
                }
            ))
        );
    }
}
