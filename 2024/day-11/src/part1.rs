use common::custom_error::Result;
use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;

pub fn process(input: &str) -> Result<String> {
    let (_, mut stones) = parse_input(input).expect("Should parse!");

    // println!("Initial arrangement:\n{:?}\n", stones);

    for _i in 1..26 {
        stones = arrange_stones(&stones);
        // println!("After {} blinks:\n{:?}\n", i, stones);
    }

    Ok(stones.len().to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn arrange_stones(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = vec![];

    for stone in stones.iter() {
        if stone == &0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            // split stones
            let temp = stone.to_string();
            let length = temp.len() / 2;
            new_stones.push((temp[0..length]).parse::<u64>().unwrap());
            new_stones.push((temp[length..]).parse::<u64>().unwrap());
        } else {
            new_stones.push(*stone * 2024);
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
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
