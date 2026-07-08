use crate::{Instruction, Parameter, parse_input};
use common::custom_error::Result;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, instructions) = parse_input(input)?;

    let queue0: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));
    let queue1: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));

    let mut machine0 = AsyncMachine::new(&instructions, 0, Rc::clone(&queue0), Rc::clone(&queue1));
    let mut machine1 = AsyncMachine::new(&instructions, 1, Rc::clone(&queue1), Rc::clone(&queue0));

    let mut done = false;
    while !done {
        while !machine0.is_await {
            machine0.step();
        }

        while !machine1.is_await {
            machine1.step();
        }

        // let each machine know if there is something waiting or not
        machine0.is_await = queue1.borrow().is_empty();
        machine1.is_await = queue0.borrow().is_empty();

        // check for deadlock
        if machine0.is_await && machine1.is_await {
            done = true;
        }
    }

    Ok(machine1.recover.to_string())
}

pub struct AsyncMachine<'a> {
    id: i64,
    pc: usize,
    registers: HashMap<char, i64>,
    recover: i64,
    instructions: &'a Vec<Instruction>,
    halted: bool,
    is_jump: bool,
    pub is_await: bool,
    send_queue: Rc<RefCell<VecDeque<i64>>>,
    receive_queue: Rc<RefCell<VecDeque<i64>>>,
}

impl<'a> AsyncMachine<'a> {
    pub fn new(instructions: &'a Vec<Instruction>, id: i64, send_queue: Rc<RefCell<VecDeque<i64>>>, receive_queue: Rc<RefCell<VecDeque<i64>>>) -> AsyncMachine<'a> {
        let mut machine = AsyncMachine {
            id,
            pc: 0,
            registers: HashMap::default(),
            recover: 0,
            instructions,
            halted: false,
            is_jump: false,
            is_await: false,
            send_queue,
            receive_queue,
        };

        machine.set_register(&'p', id);

        machine
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            return true;
        }

        let ins = self.instructions[self.pc];
        info!("{} : {:?}", &self.id, &ins);

        match ins {
            Instruction::Set(x, y) => self.set(&x, &y),
            Instruction::Add(x, y) => self.add(&x, &y),
            Instruction::Mul(x, y) => self.mul(&x, &y),
            Instruction::Mod(x, y) => self.div(&x, &y),
            Instruction::Jump(x, y) => self.jump(&x, &y),
            Instruction::Send(x) => self.send(&x),
            Instruction::Receive(x) => self.receive(&x),
        };

        if !self.is_jump && !self.is_await {
            self.pc += 1;
        }
        else {
            self.is_jump = false;
        }

        info!("{} : PC: {}, REG: {:?}, Send Count: {}", &self.id, &self.pc, &self.registers, &self.recover);

        self.halted
    }

    fn receive(&mut self, param1: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            if self.receive_queue.borrow_mut().is_empty() {
                self.is_await = true;
                info!("{} : Waiting...", &self.id);
            } else {
                let value = self.receive_queue.borrow_mut().pop_front().unwrap();
                self.set_register(reg, value);
                info!("{} : Received:{}", &self.id, value);
                self.is_await = false;
            }
        }
    }

    fn send(&mut self, param1: &Parameter) {
        let operand = match param1 {
            Parameter::Register(reg) => self.get_register(reg),
            Parameter::Value(val) => *val,
        };

        self.send_queue.try_borrow_mut().unwrap().push_back(operand);
        self.recover += 1;
    }

    fn jump(&mut self, param1: &Parameter, param2: &Parameter) {
        if let Parameter::Value(offset) = param2 {
            let condition = match param1 {
                Parameter::Register(reg2) => self.get_register(reg2),
                Parameter::Value(val) => *val,
            };

            if condition > 0 {
                self.pc = (self.pc as i64 + offset) as usize;
                self.is_jump = true;
            }
        }
    }

    fn set(&mut self, param1: &Parameter, param2: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let operand = match param2 {
                Parameter::Register(reg2) => self.get_register(reg2),
                Parameter::Value(val) => *val,
            };
            self.set_register(reg, operand);
        }
    }

    fn add(&mut self, param1: &Parameter, param2: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let operand1 = self.get_register(reg);
            let operand2 = match param2 {
                Parameter::Register(reg2) => self.get_register(reg2),
                Parameter::Value(val) => *val,
            };
            self.set_register(reg, operand1 + operand2);
        }
    }

    fn mul(&mut self, param1: &Parameter, param2: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let operand1 = self.get_register(reg);
            let operand2 = match param2 {
                Parameter::Register(reg2) => self.get_register(reg2),
                Parameter::Value(val) => *val,
            };
            self.set_register(reg, operand1 * operand2);
        }
    }

    fn div(&mut self, param1: &Parameter, param2: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let operand1 = self.get_register(reg);
            let operand2 = match param2 {
                Parameter::Register(reg2) => self.get_register(reg2),
                Parameter::Value(val) => *val,
            };

            if operand2 > 0 {
                self.set_register(reg, operand1 % operand2);
            }
        }
    }

    fn get_register(&mut self, register: &char) -> i64 {
        //self.registers.get(register).copied().unwrap_or(0)
        self.registers.entry(register.clone()).or_insert(0).clone()
    }

    fn set_register(&mut self, register: &char, value: i64) {
        self.registers.insert(*register, value);
    }
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
