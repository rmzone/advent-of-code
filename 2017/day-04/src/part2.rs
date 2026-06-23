use crate::parse_input;
use common::custom_error::Result;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, pass_phrases) = parse_input(input)?;

    let result = pass_phrases
        .iter()
        .fold(0, |acc, phrase| acc + is_valid(&phrase));

    Ok(result.to_string())
}

fn is_valid(phrase: &Vec<&str>) -> i32 {
    let count = phrase.iter().combinations(2).fold(0, |acc, comb| {
        // info!("{:?} {:?}", &comb[0], &comb[1]);
        if sort_by_foo(comb[0]) == sort_by_foo(comb[1]) {
            return acc + 1;
        }

        acc
    });

    if count > 0 { 0 } else { 1 }
}

fn sort_by_foo(input: &str) -> Vec<char> {
    let ch = input.chars().sorted().collect();

    // println!("{:?}", &ch);

    ch
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "abcde fghij
abcde xyz ecdab
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_is_valid() {
        let input = "ecdab";
        let result = sort_by_foo(input);
        assert_eq!(vec!['a', 'b', 'c', 'd', 'e'], result);
    }
}
