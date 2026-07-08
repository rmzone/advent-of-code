use nom::character::complete;
use nom::character::complete::{alpha1, alphanumeric1, line_ending, one_of, space0, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use std::collections::HashMap;
use tracing::info;

pub mod part1;
pub mod part2;

pub struct Machine {
    pc: usize,
    registers: HashMap<char, i64>,
    recover: i64,
    instructions: Vec<Instruction>,
    halted: bool,
    is_jump: bool,
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Machine {
        Machine {
            pc: 0,
            registers: HashMap::default(),
            recover: 0,
            instructions,
            halted: false,
            is_jump: false,
        }
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
            Instruction::Send(x) => self.play_sound(&x),
            Instruction::Receive(x) => self.recover(&x),
        };

        if !self.is_jump {
            self.pc += 1;
        }
        else {
            self.is_jump = false;
        }

        info!("PC: {} , REG: {:?}", &self.pc, &self.registers);

        self.halted
    }

    fn recover(&mut self, param1: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let value = self.get_register(reg);
            if value > 0 {
                self.halted = true;
            }
        }
    }

    fn play_sound(&mut self, param1: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let value = self.get_register(reg);
            self.recover = value;
        }
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
