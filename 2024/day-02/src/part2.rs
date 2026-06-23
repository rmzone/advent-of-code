use crate::{check_safe_loop, parse_input};
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, output) = parse_input(input).expect("should parse");
    let count = output.iter().filter(|o| check_safe_loop(o)).count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
