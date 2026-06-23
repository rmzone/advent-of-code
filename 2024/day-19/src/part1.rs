use crate::parse_input;
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, (fragments, towels)) = parse_input(input).expect("Should parse!");

    let result = towels
        .iter()
        .filter(|&&towel| can_build_towel(towel, &fragments))
        .count();

    Ok(result.to_string())
}

fn can_build_towel(towel: &str, fragments: &[&str]) -> bool {
    let mut stack: Vec<String> = vec!["".to_string()];
    while let Some(item) = stack.pop() {
        for fragment in fragments {
            let pattern = format!("{}{}", item, fragment);

            if pattern == towel {
                return true;
            }

            if towel.starts_with(&pattern) {
                stack.push(pattern);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
