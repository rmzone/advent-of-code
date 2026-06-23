use crate::find_shortest_sequence;
use common::custom_error::Result;

pub fn process(input: &str) -> Result<String> {
    let result: usize = input
        .lines()
        .map(|line| {
            find_shortest_sequence(line.to_string(), 2, true)
                * line.trim_end_matches('A').parse::<usize>().unwrap()
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(input)?);
        Ok(())
    }
}
