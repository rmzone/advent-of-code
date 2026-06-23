use common::custom_error::Result;
use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;
use num_traits::Euclid;
use std::collections::HashMap;

pub fn process(input: &str, blinks: u64) -> Result<String> {
    let (_, stones) = parse_input(input).expect("Should parse!");

    // trick: we don't care about the order of elements, only the counts by each unique number.
    let mut cache: HashMap<u64, u64> = HashMap::default();

    // first pass build an initial cache
    for stone in stones {
        cache.entry(stone).and_modify(|e| *e += 1).or_insert(1);
    }

    for _ in 0..blinks {
        cache = arrange_stones(&cache);
        // println!("{:?}", cache);
    }

    Ok(cache.values().sum::<u64>().to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn arrange_stones(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::default();

    for (&stone, &count) in stones {
        if stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
            let num_digits = stone.checked_ilog10().unwrap_or(0) + 1;
            let (left, right) = stone.div_rem_euclid(&10u64.pow(num_digits / 2));

            new_stones
                .entry(left)
                .and_modify(|e| *e += count)
                .or_insert(count);
            new_stones
                .entry(right)
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else {
            new_stones
                .entry(stone * 2024)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
    }

    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input, 25)?);
        Ok(())
    }
}
