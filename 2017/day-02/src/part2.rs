use crate::parse_input;
use common::custom_error::Result;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, rows) = parse_input(input)?;

    let result = rows
        .into_iter()
        .fold(0, |acc, row| acc + find_even_divisor(&row));

    Ok(result.to_string())
}

/// chose two pairs
/// divide largest / smallest
/// if no remainder then add result
fn find_even_divisor(row: &Vec<i32>) -> i32 {
    row.iter()
        .sorted_by(|a, b| b.cmp(a))
        .combinations(2)
        .fold(0, |mut acc, u| {
            let a = u[0];
            let b = u[1];

            if a.rem_euclid(*b) == 0 {
                acc += a / *b;
            }

            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!("9", process(input)?);
        Ok(())
    }

    #[test]
    fn test_find_even_divisor() {
        let input = vec![9, 4, 7, 3];
        assert_eq!(3, find_even_divisor(&input));
    }
}
