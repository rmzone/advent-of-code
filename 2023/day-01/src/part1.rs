use common::custom_error::AocError;

fn extract_int(text: &str) -> i32 {
    let numbers: Vec<i32> = text
        .chars()
        .filter_map(|x| x.to_string().parse::<i32>().ok())
        .collect();
    let first = numbers.first().unwrap_or(&0);
    let last = numbers.last().unwrap_or(&0);

    first * 10 + last
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let calibrations = input.lines().map(|l| extract_int(l)).collect::<Vec<i32>>();
    let total = calibrations.iter().sum::<i32>();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
