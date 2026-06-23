use crate::{find_middle, is_valid, parse_input};
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, (rules, updates)) = parse_input(input).expect("should parse");
    let mut count = 0;

    for update in updates {
        if is_valid(&update, &rules) {
            count += find_middle(&update);
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
