use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Computer {
    pub ip: u64,
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
    pub program: Vec<u8>,
    pub output: Vec<u8>,
}

impl Computer {
    pub fn new(reg_a: u64, reg_b: u64, reg_c: u64, program: Vec<u8>) -> Computer {
        Computer {
            ip: 0,
            reg_a,
            reg_b,
            reg_c,
            program,
            output: vec![],
        }
    }

    pub fn execute(&mut self) -> bool {
        let next = self.fetch();
        if next.is_none() {
            return true;
        }

        let (instruction, operand) = next.unwrap();

        match instruction {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Unexpected instruction"),
        }

        false
    }

    pub fn get_output(self) -> String {
        let mut output = String::new();
        for c in self.output.into_iter() {
            if !output.is_empty() {
                output.push(',');
            }
            output.push_str(&format!("{}", c));
        }

        output
    }

    fn fetch(&mut self) -> Option<(u8, u8)> {
        if self.ip + 2 <= self.program.len() as u64 {
            let instruction = self.program[self.ip as usize];
            let operand = self.program[self.ip as usize + 1];
            self.ip += 2;
            Some((instruction, operand))
        } else {
            None
        }
    }

    fn decode_combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unexpected operand value"),
        }
    }

    /// The adv instruction (opcode 0) performs
    /// division. The numerator is the value in
    /// the A register. The denominator is found
    /// by raising 2 to the power of the
    /// instruction's combo operand. (So, an
    /// operand of 2 would divide A by 4 (2^2); an
    /// operand of 5 would divide A by 2^B.) The
    /// result of the division operation is
    /// truncated to an integer and then written
    /// to the A register.
    fn adv(&mut self, operand: u8) {
        let arg = self.decode_combo(operand) as u32;
        let denom = u64::pow(2, arg);
        let result = self.reg_a / denom;
        self.reg_a = result;
    }

    /// The bxl instruction (opcode 1) calculates
    /// the bitwise XOR of register B and the
    /// instruction's literal operand, then stores
    /// the result in register B.
    fn bxl(&mut self, operand: u8) {
        let result = self.reg_b ^ operand as u64;
        self.reg_b = result;
    }

    /// The bst instruction (opcode 2) calculates
    /// the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits),
    /// then writes that value to the B register.
    fn bst(&mut self, operand: u8) {
        let arg = self.decode_combo(operand) % 8;
        self.reg_b = arg;
    }

    /// The jnz instruction (opcode 3) does
    /// nothing if the A register is 0. However,
    /// if the A register is not zero, it jumps by
    /// setting the instruction pointer to the
    /// value of its literal operand; if this
    /// instruction jumps, the instruction pointer
    /// is not increased by 2 after this
    /// instruction.
    fn jnz(&mut self, operand: u8) {
        if self.reg_a != 0 {
            self.ip = operand as u64;
        }
    }

    /// The bxc instruction (opcode 4) calculates
    /// the bitwise XOR of register B and register
    /// C, then stores the result in register B.
    /// (For legacy reasons, this instruction
    /// reads an operand but ignores it.)
    fn bxc(&mut self, _operand: u8) {
        let result = self.reg_b ^ self.reg_c;
        self.reg_b = result;
    }

    /// The out instruction (opcode 5) calculates
    /// the value of its combo operand modulo 8,
    /// then outputs that value. (If a program
    /// outputs multiple values, they are
    /// separated by commas.)
    fn out(&mut self, operand: u8) {
        let arg = (self.decode_combo(operand) % 8) as u8;
        self.output.push(arg);
        // println!("{:#032b}", &self.reg_a);
    }

    /// The bdv instruction (opcode 6) works
    /// exactly like the adv instruction except
    /// that the result is stored in the B
    /// register. (The numerator is still read
    /// from the A register.)
    fn bdv(&mut self, operand: u8) {
        let arg = self.decode_combo(operand);
        let denom = u64::pow(2, arg as u32);
        let result = self.reg_a / denom;
        self.reg_b = result;
    }

    /// The cdv instruction (opcode 7) works
    /// exactly like the adv instruction except
    /// that the result is stored in the C
    /// register. (The numerator is still read
    /// from the A register.)
    fn cdv(&mut self, operand: u8) {
        let arg = self.decode_combo(operand);
        let denom = u64::pow(2, arg as u32);
        let result = self.reg_a / denom;
        self.reg_c = result;
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Computer> {
    let (input, reg_a) =
        preceded(tag("Register A: "), terminated(complete::u64, line_ending))(input)?;

    let (input, reg_b) =
        preceded(tag("Register B: "), terminated(complete::u64, line_ending))(input)?;

    let (input, reg_c) =
        preceded(tag("Register C: "), terminated(complete::u64, line_ending))(input)?;

    let (input, program) = preceded(tuple((line_ending, tag("Program: "))), parse_set)(input)?;

    Ok((input, Computer::new(reg_a, reg_b, reg_c, program)))
}

fn parse_set(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), complete::u8)(input)
}
