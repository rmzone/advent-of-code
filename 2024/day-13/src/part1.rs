use common::custom_error::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::IResult;
use std::cmp::min;

#[derive(Debug, PartialEq)]
struct Button {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Prize {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

pub fn process(input: &str) -> Result<String> {
    let (_, machines) = process_input(input).expect("Should parse!");
    let result = machines.iter().map(calculate_tokens).sum::<i32>();

    Ok(result.to_string())
}

fn calculate_tokens(machine: &Machine) -> i32 {
    let mut token_cost = i32::MAX;

    for a in 1..=100 {
        for b in 1..=100 {
            let candidate_x = a * machine.button_a.x + b * machine.button_b.x;
            let candidate_y = a * machine.button_a.y + b * machine.button_b.y;

            if candidate_x == machine.prize.x && candidate_y == machine.prize.y {
                let new_cost = a * 3 + b;
                token_cost = min(token_cost, new_cost);
            }
        }
    }

    if token_cost == i32::MAX {
        token_cost = 0;
    }

    println!("Machine: {:?} -> {}", machine.prize, token_cost);

    token_cost
}

fn process_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tuple((line_ending, line_ending)), process_machine)(input)
}

fn process_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = terminated(process_button, line_ending)(input)?;
    let (input, button_b) = terminated(process_button, line_ending)(input)?;
    let (input, prize) = process_prize(input)?;

    Ok((
        input,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn process_button(input: &str) -> IResult<&str, Button> {
    let (input, _) = alt((tag("Button A: X+"), tag("Button B: X+")))(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = complete::i32(input)?;

    Ok((input, Button { x, y }))
}

fn process_prize(input: &str) -> IResult<&str, Prize> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = complete::i32(input)?;

    Ok((input, Prize { x, y }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_machine() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        assert_eq!(
            process_machine(input),
            Ok((
                "",
                Machine {
                    button_a: Button { x: 94, y: 34 },
                    button_b: Button { x: 22, y: 67 },
                    prize: Prize { x: 8400, y: 5400 }
                }
            ))
        );
    }

    #[test]
    fn test_parse_button() {
        let input = "Button A: X+94, Y+34";
        assert_eq!(process_button(input), Ok(("", Button { x: 94, y: 34 })));
    }

    #[test]
    fn test_parse_prize() {
        let input = "Prize: X=8400, Y=5400";
        assert_eq!(process_prize(input), Ok(("", Prize { x: 8400, y: 5400 })));
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
