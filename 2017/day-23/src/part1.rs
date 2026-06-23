use crate::{Machine, parse_input};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, instructions) = parse_input(input)?;
    let mut machine = Machine::new(&instructions);

    info!("{:?}", &instructions);

    let mut done = false;
    while !done {
        done = machine.step();
    }

    Ok(machine.mul_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "set b 84
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23";
        assert_eq!("6724", process(input)?);
        Ok(())
    }
}
