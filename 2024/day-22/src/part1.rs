use common::custom_error::Result;

pub fn process(input: &str, count: i32) -> Result<String> {
    let seeds: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut result = 0u64;

    for seed in seeds {
        let mut secret = seed;

        for i in 0..count {
            let temp = secret * 64;
            secret ^= temp;
            secret %= 16777216;

            let temp = secret / 32;
            secret ^= temp;
            secret %= 16777216;

            let temp = secret * 2048;
            secret ^= temp;
            secret %= 16777216;

            // println!("{}", secret);
        }

        result += secret;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input, 2000)?);
        Ok(())
    }

    #[test]
    fn test_sample() -> Result<()> {
        let input = "123";
        assert_eq!("5908254", process(input, 10)?);
        Ok(())
    }
}
