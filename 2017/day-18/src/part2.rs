use crate::{parse_input, Machine};
use common::custom_error::Result;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, instructions) = parse_input(input)?;
    let queue0: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));
    let queue1: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));

    let mut machine0 = Machine::new(&instructions, 0, Rc::clone(&queue0), Rc::clone(&queue1));
    let mut machine1 = Machine::new(&instructions, 1, Rc::clone(&queue1), Rc::clone(&queue0));

    let mut done = false;
    while !done {
        while !machine0.step() {}

        while !machine1.step() {}

        // check for deadlock
        if machine0.step() {
            done = true;
        }
    }

    Ok(machine1.send_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
