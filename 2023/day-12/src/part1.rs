use itertools::Itertools;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete;
use nom::character::complete::line_ending;

use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use common::custom_error::AocError;

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub line: &'a str,
    pub damaged: Vec<i32>,
    spaces_to_fill: i32
}

fn parse_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), complete::i32)(input)
}

fn parse_line(input: &str) -> IResult<&str, Puzzle> {
    let (input, (line, damaged)) = separated_pair(
        is_a(".#?"), tag(" "), parse_list)(input)?;

    let spaces_to_fill = line.chars().filter(|c| c== &'?').count() as i32;

    Ok((input, Puzzle { line, damaged, spaces_to_fill }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Puzzle>> {
    separated_list1(line_ending, parse_line)(input)
}

fn validate(expected: &[i32], value: &str) -> bool {
    let mut damaged: Vec<i32> = Vec::new();

    let foo = value.split('.');
    for f in foo {
        let count = f.len();
        // dbg!(f, count);
        if count > 0 {
            damaged.push(count as i32);
        }
    }

    let matching = damaged.iter().zip(expected.iter()).filter(|&(a, b)| a == b).count();
    matching == damaged.len() && matching == expected.len()
}

fn process_line(puzzle: &Puzzle) -> Option<i32> {
    let mut count = 0;

    let mut test = String::new();
    let characters = vec![".", "#"];

    let combinations : Vec<_> = (2..puzzle.spaces_to_fill).fold(
        characters.iter().cartesian_product(characters.iter()).map(|(&a, &b)| a.to_owned() + b).collect(),
        |acc, _| acc.into_iter().cartesian_product(characters.iter()).map(|(a, b)| a.to_owned() + b).collect()
    );

    for mut filler in combinations {
        test.clear();

        for c in puzzle.line.chars() {
            if c == '?' {
                test.push(filler.pop()?);
            }
            else {
                test.push(c);
            }
        }

        if validate(&puzzle.damaged, &test) {
            count += 1;
        }

      //  dbg!(&test/*, &count*/);
    }

    Some(count)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, lines) = parse_lines(input).unwrap();
    let result = lines.iter().map(|line| {
        process_line(line).unwrap()
    }).sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}

