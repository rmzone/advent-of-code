use nom::character::complete;
use nom::character::complete::{alphanumeric1, line_ending, one_of, space0, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use tracing::info;

pub mod part1;
pub mod part2;

pub struct Machine<'a> {
    id: i64,
    pc: usize,
    registers: HashMap<char, i64>,
    instructions: &'a Vec<Instruction>,
    halted: bool,
    pub is_await: bool,
    send_queue: Rc<RefCell<VecDeque<i64>>>,
    receive_queue: Rc<RefCell<VecDeque<i64>>>,
    pub send_count: i64,
    pub last_sent: i64,
    pub first_non_zero_recv: i64,
}

impl<'a> Machine<'a> {
    pub fn new(
        instructions: &'a Vec<Instruction>,
        id: i64,
        send_queue: Rc<RefCell<VecDeque<i64>>>,
        receive_queue: Rc<RefCell<VecDeque<i64>>>,
    ) -> Machine<'a> {
        let mut machine = Machine {
            id,
            pc: 0,
            registers: HashMap::default(),
            send_count: 0,
            instructions,
            receive_queue,
            send_queue,
            halted: false,
            is_await: false,
            last_sent: 0,
            first_non_zero_recv: 0,
        };

        machine.set_register(&'p', id);

        machine
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            return true;
        }

        let ins = self.instructions[self.pc];
        info!("{:?}", &ins);

        match ins {
            Instruction::Set(x, y) => self.set(&x, &y),
            Instruction::Add(x, y) => self.add(&x, &y),
            Instruction::Mul(x, y) => self.mul(&x, &y),
            Instruction::Mod(x, y) => self.div(&x, &y),
            Instruction::Jump(x, y) => self.jump(&x, &y),
            Instruction::Send(x) => self.send(&x),
            Instruction::Receive(x) => self.receive(&x),
        };

        self.pc += 1;

        info!(
            "{} : PC: {}, REG: {:?}, Send Count: {}",
            &self.id, &self.pc, &self.registers, &self.send_count
        );

        self.is_await || self.halted
    }

    fn receive(&mut self, param1: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            if self.first_non_zero_recv == 0 && self.get_register(reg) != 0 {
                self.first_non_zero_recv = self.last_sent;
            }

            if self.receive_queue.borrow_mut().is_empty() {
                self.is_await = true;
                self.pc -= 1;
            } else {
                let value = self.receive_queue.borrow_mut().pop_front().unwrap();
                self.set_register(reg, value);
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
        self.send_count += 1;
        self.last_sent = operand;
    }

    fn jump(&mut self, param1: &Parameter, param2: &Parameter) {
        let condition = match param1 {
            Parameter::Register(reg) => self.get_register(reg),
            Parameter::Value(val) => *val,
        };

        let offset = match param2 {
            Parameter::Register(reg) => self.get_register(reg),
            Parameter::Value(val) => *val,
        };

        if condition > 0 {
            self.pc = (self.pc as i64 + offset - 1) as usize;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parameter {
    Register(char),
    Value(i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Set(Parameter, Parameter),
    Add(Parameter, Parameter),
    Mul(Parameter, Parameter),
    Mod(Parameter, Parameter),
    Jump(Parameter, Parameter),
    Send(Parameter),
    Receive(Parameter),
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (ins, _, param1, _, param2)) =
        (alphanumeric1, space1, parameter, space0, opt(parameter)).parse(input)?;

    let instruction = match ins {
        "set" => Instruction::Set(param1, param2.unwrap()),
        "add" => Instruction::Add(param1, param2.unwrap()),
        "mul" => Instruction::Mul(param1, param2.unwrap()),
        "mod" => Instruction::Mod(param1, param2.unwrap()),
        "jgz" => Instruction::Jump(param1, param2.unwrap()),
        "snd" => Instruction::Send(param1),
        "rcv" => Instruction::Receive(param1),
        _ => panic!("invalid instruction"),
    };

    Ok((input, instruction))
}

fn parameter(input: &str) -> IResult<&str, Parameter> {
    let tokens: String = ('a'..='z').into_iter().map(|c| c.to_string()).collect();
    let (input, (r, i)) = (opt(one_of(tokens.as_str())), opt(complete::i64)).parse(input)?;

    if r.is_some() {
        return Ok((input, Parameter::Register(r.unwrap())));
    }

    if i.is_some() {
        return Ok((input, Parameter::Value(i.unwrap())));
    }

    Ok((input, Parameter::Value(999999)))
    // panic!("invalid parameter");
}
