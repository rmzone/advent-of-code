use crate::{parse_input, Computer};
use common::custom_error::{Error, Result};
use itertools::equal;

pub fn process(input: &str) -> Result<String, Error> {
    let (_, computer) = parse_input(input).expect("Should parse!");
    let result = find_register(&computer);

    // brute force takes too long. need a way to pick numbers
    // but analyze the program. work backwards.
    //
    // 2,4, bst(A)
    // 1,7, bxl(7)
    // 7,5, cdv(B)
    // 0,3, adv(3)
    // 4,0, bxc()
    // 1,7, bxl(7)
    // 5,5, out(B)
    // 3,0 jnz(0)

    Ok(result.to_string())
}

fn test_guess(start_computer: &Computer, guess: u64) -> (bool, String) {
    let mut computer = start_computer.clone();
    computer.reg_a = guess;

    loop {
        let should_halt = computer.execute();
        if should_halt {
            break;
        }
    }

    (
        equal(&computer.program, &computer.output),
        computer.get_output(),
    )
}
fn simulate_loop(start_computer: &Computer, guess: u64) -> u64 {
    let mut computer = start_computer.clone();
    computer.reg_a = guess;

    for _ in 0..computer.program.len() {
        computer.execute();
    }

    computer.output[0] as u64
}

fn find_register(computer: &Computer) -> u64 {
    let mut answers = Vec::new();
    let mut stack = vec![(0, computer.program.len())];
    while let Some((a, depth)) = stack.pop() {
        // Try all possible values for b and push ones that produce the correct
        // result on the stack, shifting by 3 bits each time.
        for b in 0..8 {
            let temp_a = (a << 3) | b as u64;
            println!("testing: :{:#o}", &temp_a);

            if depth == 0 {
                continue;
            }

            if simulate_loop(computer, temp_a) == (computer.program[depth - 1]) as u64 {
                let sample = test_guess(computer, temp_a);
                println!("{:#o} {} [{}]", temp_a, depth - 1, sample.1);
                if sample.0 {
                    answers.push(temp_a);
                }
                stack.push((temp_a, depth - 1));
            }
        }

        println!("stack: :{:?}\n", &stack);
    }

    println!("\nanswers: :{:?}\n", &answers);

    answers.into_iter().min().unwrap_or(0u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", process(input)?);
        Ok(())
    }
}
