use crate::parse_input;
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut jumps) = parse_input(input)?;

    let mut steps = 0;
    let mut position: i32 = 0;

    while true {
        steps += 1;

        let jump = jumps[position as usize];

        // exit condition
        if position + jump >= jumps.len() as i32 {
            break;
        }

        jumps[position as usize] = jump + 1;

        // update position
        position += jump;
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0
3
0
1
-3";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
