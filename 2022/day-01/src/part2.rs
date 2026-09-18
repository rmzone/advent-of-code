use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let lines = input.lines();

    let mut current_calories = 0;
    let mut calorie_counts = vec![];

    for line in lines {
        if line == "" {
            calorie_counts.push(current_calories);
            current_calories = 0;
            continue;
        }

        let x = line.parse::<i32>().unwrap();
        current_calories += x;
    }

    // push the last item
    calorie_counts.push(current_calories);

    calorie_counts.sort_by(|a, b| b.cmp(a));
    let biggest_calories = calorie_counts.iter().take(3).sum::<i32>();

    Ok(biggest_calories.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        assert_eq!("45000", process(input)?);
        Ok(())
    }
}
