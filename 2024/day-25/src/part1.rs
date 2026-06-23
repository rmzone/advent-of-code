use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::{iterator, opt, peek};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use common::custom_error::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum TumblerType {
    Lock,
    Key
}

#[derive(Debug)]
pub struct Tumbler {
    pub tumbler_type: TumblerType,
    pub pins: [i32; 5]
}

pub fn process(input: &str) -> Result<String> {
    let (_, tumblers) = parse_input(input).expect("failed to parse input");
    // println!("{:?}", &tumblers);

    // separate into locks and keys
    // do a cartesian product on all key lock pairs and check if they fit
    /*
    Tumbler { tumbler_type: Lock, pins: [0, 5, 3, 4, 3] },
    Tumbler { tumbler_type: Lock, pins: [1, 2, 0, 5, 3] },
    
    Tumbler { tumbler_type: Key, pins: [5, 0, 2, 1, 3] },
    Tumbler { tumbler_type: Key, pins: [4, 3, 4, 0, 2] },
    Tumbler { tumbler_type: Key, pins: [3, 0, 2, 0, 1] }
    */

    let locks: Vec<&Tumbler> = tumblers.iter().filter(|&a| a.tumbler_type == TumblerType::Lock).collect();
    let keys: Vec<&Tumbler> = tumblers.iter().filter(|&a| a.tumbler_type == TumblerType::Key).collect();

    let count = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| {
            std::iter::zip(lock.pins, key.pins)
                .all(|(a, b)| a + b <= 5)
        })
        .count();

    Ok(count.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Tumbler>> {
    separated_list1(line_ending, alt((key, lock)))(input)
}

fn key(input: &str) -> IResult<&str, Tumbler> {
    let (input, _) = tag(".....")(input)?;
    let (input, pins) = preceded(
        tuple((
            line_ending,
            peek(alt((tag("."), tag("#")))),
        )),
        accumulate_pins([-1i32; 5]), // start at -1 to get the correct result
    )(input)?;

    Ok((
        input,
        Tumbler {
            pins,
            tumbler_type: TumblerType::Key,
        },
    ))
}

fn lock(input: &str) -> IResult<&str, Tumbler> {
    let (input, _) = tag("#####")(input)?;
    let (input, pins) = preceded(
        tuple((
            line_ending,
            peek(alt((tag("."), tag("#")))),
        )),
        accumulate_pins([0i32; 5]),
    )(input)?;

    Ok((
        input,
        Tumbler {
            pins,
            tumbler_type: TumblerType::Lock,
        },
    ))
}

// starts always at the second row
fn accumulate_pins(mut pins: [i32; 5]) -> impl FnMut(&str) -> IResult<&str, [i32; 5]> {
    move |input| {
        let mut it = iterator(
            input,
            terminated(
                alt((tag("#"), tag("."))),
                opt(line_ending),
            ),
        );

        for (i, value) in it.enumerate() {
            pins[i % 5] += match value {
                "#" => 1,
                _ => 0,
            };
        }

        let res: IResult<_, _> = it.finish();

        res.map(|(input, _)| (input, pins))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
