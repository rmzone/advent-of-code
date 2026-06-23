use std::collections::HashMap;
use itertools::{Itertools};
use nom::{
    bytes::complete::{tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom::character::complete::{line_ending, one_of};
use nom::multi::many1;
use common::custom_error::AocError;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => Spring::Unknown
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Record {
    springs: Vec<Spring>,
    groups: Vec<u64>
}

impl Record {
    fn new(springs: Vec<Spring>, groups: Vec<u64>) -> Self {
        Self { springs, groups }
    }

    fn expand(&self) -> Record {
        let springs = self
            .springs
            .iter()
            .cloned()
            .chain([Spring::Unknown].iter().cloned())
            .cycle()
            .take(self.springs.len() * 5 + 4)
            .collect();
        let groups = self
            .groups
            .iter()
            .cloned()
            .cycle()
            .take(self.groups.len() * 5)
            .collect();
        Record::new(springs, groups)
    }
}

fn parse_line(input: &str) -> IResult<&str, Record> {
    let (input, (springs, groups)) = separated_pair(
        many1(one_of("?.#")),
        space1,
        separated_list1(tag(","), complete::u64),
    )(input)?;

    Ok((input, Record {
        springs: springs.into_iter().map(|c| c.into()).collect(),
        groups: groups
    }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(line_ending, parse_line)(input)
}

fn possible_solutions(memo: &mut HashMap<Record, u64>, record: &Record) -> u64 {
    // Check to see if we have already calculated this record.
    if let Some(&v) = memo.get(record) {
        return v;
    }

    // If we have no groups left we have a solution if there are no
    // other damaged springs in our list. Otherwise, it can't be valid.
    if record.groups.is_empty() {
        let v = match record.springs.iter().any(|c| *c == Spring::Damaged) {
            true => 0,
            false => 1,
        };
        memo.insert(record.clone(), v);
        return v;
    }

    // At this point, we have some number of groups left, so make sure
    // we we have enough springs left to fill them.
    if record.springs.len() < record.groups.iter().sum::<u64>() as usize + record.groups.len() - 1 {
        memo.insert(record.clone(), 0);
        return 0;
    }

    // We can't do anything with operational springs. So just skip it.
    if record.springs[0] == Spring::Operational {
        let solutions = possible_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
        memo.insert(record.clone(), solutions);
        return solutions;
    }

    // At this point, we know we are at the beginning of a possible
    // position for the current group. We can check if that's possible
    // and if it is, we can see how many valid solutions we'd get if
    // we did.
    let mut solutions = 0;
    let cur = record.groups[0] as usize;
    let all_non_operational = record.springs[0..cur]
        .iter()
        .all(|c| *c != Spring::Operational);
    let end = (cur + 1).min(record.springs.len());
    if all_non_operational
        && ((record.springs.len() > cur && record.springs[cur] != Spring::Damaged)
        || record.springs.len() <= cur)
    {
        solutions = possible_solutions(
            memo,
            &Record::new(record.springs[end..].to_vec(), record.groups[1..].to_vec()),
        );
    }

    // If our current position is a unknown, we could also choose not
    // to use it, so include those possibilities.
    if record.springs[0] == Spring::Unknown {
        solutions += possible_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
    }

    // We now know the number of solutions for this record, so memoize
    // it and return it.
    memo.insert(record.clone(), solutions);
    solutions
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, records) = parse_lines(input).unwrap();
    let mut memo: HashMap<Record, u64> = HashMap::new();

    let sum = records
        .iter()
        .map(|r| possible_solutions(&mut memo, &r.expand()))
        .sum::<u64>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    #[test]
    fn test_line(
        #[case] input: &str,
        #[case] output: usize,
    ) {
        assert_eq!(output.to_string(), process(input).unwrap());
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("525152", process(input)?);
        Ok(())
    }
}

