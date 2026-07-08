use tracing::info;
use crate::{Machine, parse_input};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, instructions) = parse_input(input)?;

    // for instruction in &instructions {
    //     info!("{:?}", &instruction);
    // }

    let mut machine = Machine::new(instructions);

    while !machine.step() {
        machine.step();
    }

    Ok(machine.recover.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
