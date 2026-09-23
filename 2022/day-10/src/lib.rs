use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    AddX(i32),
}

pub struct CathodeRayTube {
    cycles: usize,
    register_x: i32,
    raster: String,
}

impl CathodeRayTube {
    pub fn new() -> CathodeRayTube {
        CathodeRayTube {
            cycles: 0,
            register_x: 1,
            raster: "".to_string(),
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> i64 {
        let mut signal_strength = 0;

        let cycle_count = match instruction {
            Instruction::Nop => 1,
            Instruction::AddX(_) => 2,
        };

        for _ in 0..cycle_count {
            // next raster
            if [40, 80, 120, 160, 200, 240]
                .iter()
                .any(|&x| x == self.cycles)
            {
                self.raster += "\n";
            }

            // output pixel
            let row: i32 = (self.cycles % 40) as i32;
            if row >= self.register_x - 1 && row <= self.register_x + 1 {
                self.raster += "#";
            } else {
                self.raster += ".";
            }

            self.cycles += 1;

            // output signal strength for specific cycles, otherwise 0
            if [20, 60, 100, 140, 180, 220]
                .iter()
                .any(|&x| x == self.cycles)
            {
                signal_strength += self.register_x * self.cycles as i32;
            }
        }

        match instruction {
            Instruction::Nop => {}
            Instruction::AddX(argument) => {
                self.register_x += argument;
            }
        }

        signal_strength as i64
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, opcode) = alt((tag("noop"), tag("addx"))).parse(input)?;
    let (input, _) = space0.parse(input)?;
    let (input, value) = opt(complete::i32).parse(input)?;

    let instruction = match opcode {
        "addx" => Instruction::AddX(value.unwrap()),
        "noop" => Instruction::Nop,
        _ => unreachable!(),
    };

    Ok((input, instruction))
}
