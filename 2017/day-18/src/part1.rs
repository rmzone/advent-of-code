use crate::{parse_input, Machine};
use common::custom_error::Result;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, instructions) = parse_input(input)?;
    let queue: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));
    let mut machine = Machine::new(&instructions, 0, Rc::clone(&queue), Rc::clone(&queue));

    let mut done = false;
    while !done {
        done = machine.step();
    }

    Ok(machine.first_non_zero_recv.to_string())
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
