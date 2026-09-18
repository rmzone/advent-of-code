use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let lines = input.lines();
    let mut biggest_calories = -1;
    let mut current_calories = 0;

    for line in lines {
        if line == "" {
            if current_calories > biggest_calories {
                biggest_calories = current_calories;
            }
            current_calories = 0;
            continue;
        }

        let x = line.parse::<i32>().unwrap();
        current_calories += x;
    }

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

10000";
        assert_eq!("24000", process(input)?);
        Ok(())
    }
}
