use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32).parse(input)
}

pub fn find_first_max(banks: &[i32]) -> Option<(usize, i32)> {
    banks
        .iter()
        .enumerate()
        .max_by_key(|&(i, &v)| (v, -(i as i64))) //ensures the highest value wins, and ties go to the lowest index (by negating i)
        .map(|(i, &v)| (i, v))
}
