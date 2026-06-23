use nom::bytes::complete::*;
use nom::character::complete::i32;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), i32).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse() {
        let input = "1,2,3";
        assert_eq!(("", vec![1, 2, 3]), parse_input(input).unwrap());
    }
}
