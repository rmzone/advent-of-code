use common::custom_error::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Secret {
    secret: u64,
    price: u64,
    delta: i64,
}

pub fn process(input: &str, count: i32) -> Result<String> {
    let seeds: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut counts: HashMap<Vec<i64>, u64> = HashMap::new();

    for seed in seeds {
        let (last_secret, secrets) = calculate_secrets(seed, count);
        let sequences = find_sequence(&secrets);

        for (sequence, price) in sequences.iter() {
            let key = sequence.to_vec();
            *counts.entry(key).or_insert(0) += price;
        }
    }

    let result = counts.iter().map(|(_, &v)| v).max().unwrap_or(0);

    Ok(result.to_string())
}

fn find_sequence(secrets: &[Secret]) -> HashMap<Vec<i64>, u64> {
    let mut sequences = HashMap::new();
    let slice = secrets.windows(5);

    for piece in slice {
        let mut v = Vec::new();
        for i in 1..5 {
            v.push(piece[i].delta)
        }

        sequences.entry(v).or_insert(piece[4].price);
    }

    sequences
}

fn calculate_secrets(seed: u64, count: i32) -> (u64, Vec<Secret>) {
    let mut secret = seed;
    let mut secrets = Vec::with_capacity(count as usize + 1);
    let mut last_price = secret % 10;

    secrets.push(Secret {
        secret,
        price: last_price,
        delta: 0,
    });

    for i in 0..count {
        secret = next_secret(secret);
        let new_price = secret % 10;
        secrets.push(Secret {
            secret,
            price: new_price,
            delta: new_price as i64 - last_price as i64,
        });
        last_price = new_price;
    }

    (secret, secrets)
}

fn next_secret(mut secret: u64) -> u64 {
    let value = secret * 64;
    secret = prune(mix(secret, value));

    let value = secret / 32;
    secret = prune(mix(secret, value));

    let value = secret * 2048;
    secret = prune(mix(secret, value));

    secret
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(value: u64) -> u64 {
    // value % 16777216
    value.rem_euclid(16777216)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", process(input, 2000)?);
        Ok(())
    }

    #[test]
    fn test_sample() -> Result<()> {
        let input = "123";
        assert_eq!("6", process(input, 10)?);
        Ok(())
    }

    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }

    #[test]
    fn test_prune() {
        assert_eq!(16113920, prune(100000000));
    }
}
