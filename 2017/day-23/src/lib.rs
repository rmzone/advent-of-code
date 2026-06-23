use nom::character::complete;
use nom::character::complete::{alphanumeric1, line_ending, one_of, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use std::collections::HashMap;
use tracing::info;

pub mod part1;
pub mod part2;

pub struct Machine<'a> {
    pc: usize,
    registers: HashMap<char, i64>,
    instructions: &'a Vec<Instruction>,
    halted: bool,
    pub mul_count: i64,
}

impl<'a> Machine<'a> {
    pub fn new(instructions: &'a Vec<Instruction>) -> Machine<'a> {
        let machine = Machine {
            pc: 0,
            registers: HashMap::default(),
            mul_count: 0,
            instructions,
            halted: false,
        };

        machine
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            self.halted = true;
            return true;
        }

        let ins = self.instructions[self.pc];
        info!("{:?}", &ins);

        match ins {
            Instruction::Set(x, y) => self.set(&x, &y),
            Instruction::Sub(x, y) => self.sub(&x, &y),
            Instruction::Mul(x, y) => self.mul(&x, &y),
            Instruction::Jump(x, y) => self.jump(&x, &y),
        };

        self.pc += 1;

        self.halted
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

        if condition != 0 {
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

    fn sub(&mut self, param1: &Parameter, param2: &Parameter) {
        if let Parameter::Register(reg) = param1 {
            let operand1 = self.get_register(reg);
            let operand2 = match param2 {
                Parameter::Register(reg2) => self.get_register(reg2),
                Parameter::Value(val) => *val,
            };
            self.set_register(reg, operand1 - operand2);
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
            self.mul_count += 1;
        }
    }

    fn get_register(&mut self, register: &char) -> i64 {
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
    Sub(Parameter, Parameter),
    Mul(Parameter, Parameter),
    Jump(Parameter, Parameter),
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (ins, _, param1, _, param2)) =
        (alphanumeric1, space1, parameter, space1, parameter).parse(input)?;

    let instruction = match ins {
        "set" => Instruction::Set(param1, param2),
        "sub" => Instruction::Sub(param1, param2),
        "mul" => Instruction::Mul(param1, param2),
        "jnz" => Instruction::Jump(param1, param2),
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
