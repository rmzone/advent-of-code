use crate::{parse_input, Condition, Operation};
use common::custom_error::Result;
use log::info;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, instructions) = parse_input(input)?;
    info!("{:?}", instructions);

    let mut highest = 0;

    // run all instructions
    let mut registers: HashMap<&str, i32> = HashMap::new();
    for instruction in instructions {
        let condition_register = registers
            .entry(&instruction.condition_lhs)
            .or_insert(0)
            .clone();
        let condition_check = instruction.condition_rhs;

        let condition = match instruction.condition_type {
            Condition::GreaterThan => condition_register > condition_check,
            Condition::LessThan => condition_register < condition_check,
            Condition::EqualTo => condition_register == condition_check,
            Condition::LessThanOrEqualTo => condition_register <= condition_check,
            Condition::GreaterThanOrEqualTo => condition_register >= condition_check,
            Condition::NotEqualTo => condition_register != condition_check,
        };

        let register = registers.entry(&instruction.register).or_insert(0);

        if condition {
            match instruction.operation {
                Operation::Inc => *register += instruction.value,
                Operation::Dec => *register -= instruction.value,
            }
        }

        // find the largest register
        let current_highest = *registers.values().max().unwrap();
        if current_highest > highest {
            info!(
                "Highest value: {} = {}",
                &instruction.register, current_highest
            );
            highest = current_highest;
        }
    }

    Ok(highest.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!("10", process(input)?);
        Ok(())
    }
}
