use common::custom_error::AocError;

fn parse_sequences(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn calculate_next(items: &Vec<i64>) -> i64 {
    if items.iter().sum::<i64>() == 0 {
        return 0;
    }

    let mut v: Vec<i64> = vec![];
    for i in 0..items.len() - 1 {
        v.push(items[i + 1] - items[i]);
    }

    items.last().unwrap() + calculate_next(&v)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sequences = parse_sequences(input);
    let result = sequences.iter().map(|u| calculate_next(u)).sum::<i64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
