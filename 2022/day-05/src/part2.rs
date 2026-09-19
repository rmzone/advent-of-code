use crate::parse_input;
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, (mut cargo, instructions)) = parse_input(input)?;

    info!("{:?} {:?}", cargo, instructions);

    for instruction in &instructions {
        cargo.move_many_crates(instruction);
    }

    let top = cargo.find_top();

    Ok(top)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("MCD", process(input)?);
        Ok(())
    }
}
