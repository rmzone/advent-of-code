use common::custom_error::AocError;
use phf::phf_map;
use substring::Substring;

static KEYWORDS: phf::Map<&'static str, i32> = phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let calibrations = input.lines().map(|l| extract_int(l)).collect::<Vec<i32>>();
    let total = calibrations.iter().sum::<i32>();

    Ok(total.to_string())
}

fn extract_int(text: &str) -> i32 {
    let mut results: Vec<i32> = vec![];

    // iterate over slices
    let len: usize = text.len();
    for i in 0..len {
        let current = text.substring(i, len);

        for number in &KEYWORDS {
            if current.starts_with(number.0) {
                results.push(*number.1);
                break;
            }
        }

        if let Ok(a) = current.substring(0, 1).parse::<i32>() {
            results.push(a);
        }
    }

    let first = results.first().unwrap_or(&0);
    let last = results.last().unwrap_or(&0);

    (*first) * 10 + (*last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
