use nom::Parser;
use common::parsing::{token, Span};
use glam::IVec2;
use nom::character::complete;
use nom::character::complete::{line_ending, one_of, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::IResult;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operation {
    Multiply,
    Add,
}

#[derive(Debug)]
pub struct Problem {
    pub operands: Vec<u64>,
    pub operation: Operation,
}

impl Problem {
    pub fn calculate(&self) -> u64 {
        match self.operation {
            Operation::Multiply => self.operands.iter().fold(1, |acc, op| acc * op),
            Operation::Add => self.operands.iter().fold(0, |acc, op| acc + op),
        }
    }
}

pub fn process_input(input: &str) -> IResult<&str, Vec<Problem>> {
    let (input, op0) = terminated(parse_ops, line_ending).parse(input)?;
    let (input, op1) = terminated(parse_ops, line_ending).parse(input)?;
    let (input, op2) = terminated(parse_ops, line_ending).parse(input)?;
    let (input, op3) = terminated(parse_ops, line_ending).parse(input)?;
    let (input, op4) = parse_oper(input)?;

    let length = op0.len();
    let mut problems: Vec<Problem> = Vec::new();

    for i in 0..length {
        let operands: Vec<u64> = vec![op0[i], op1[i], op2[i], op3[i]];
        problems.push(Problem {
            operands,
            operation: op4[i],
        });
    }

    Ok((input, problems))
}

pub fn process_input2(input: &str) -> IResult<&str, Vec<Problem>> {
    let columns = input.lines().next().unwrap().len() as i32;
    let rows = input.lines().count() as i32;

    let valid_tokens = " +*0123456789";
    let (input, cells) = separated_list1(line_ending, many1(token(valid_tokens))).parse(Span::new(input))
        .expect("should parse");
    let map =
        cells
            .into_iter()
            .flatten()
            .fold(HashMap::new(), |mut acc: HashMap<IVec2, char>, cell| {
                acc.insert(cell.0, cell.1);
                acc
            });

    // parse map to get problems
    let mut problems: Vec<Problem> = Vec::new();
    let mut args: Vec<u64> = Vec::new();

    for x in (0..columns).rev() {
        let mut arg = 0;
        for y in 0..rows {
            let cell = map.get(&IVec2::new(x, y)).unwrap_or(&' ');
            match cell {
                ' ' => {
                    if y < rows - 1 && arg <= 0 {
                        arg *= 10;
                    }
                }
                '0'..='9' => {
                    arg *= 10;
                    arg += cell.to_digit(10).unwrap()
                }
                '+' => {
                    args.push(arg as u64);
                    problems.push(Problem {
                        operands: args.clone(),
                        operation: Operation::Add,
                    });
                    args.clear();
                    arg = 0;
                }
                '*' => {
                    args.push(arg as u64);
                    problems.push(Problem {
                        operands: args.clone(),
                        operation: Operation::Multiply,
                    });
                    args.clear();
                    arg = 0;
                }
                _ => panic!(),
            }
        }

        if arg != 0 {
            args.push(arg as u64);
        }
    }

    Ok(("", problems))
}

fn parse_ops(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64).parse(input)
}

fn parse_oper(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(space1, operation).parse(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, o) = one_of("+*").parse(input)?;

    match o {
        '+' => Ok((input, Operation::Add)),
        '*' => Ok((input, Operation::Multiply)),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(
            Problem {
                operands: vec![123, 45, 6],
                operation: Operation::Multiply
            }
            .calculate(),
            33210
        );
        assert_eq!(
            Problem {
                operands: vec![328, 64, 98],
                operation: Operation::Add
            }
            .calculate(),
            490
        );
        assert_eq!(
            Problem {
                operands: vec![51, 387, 215],
                operation: Operation::Multiply
            }
            .calculate(),
            4243455
        );
        assert_eq!(
            Problem {
                operands: vec![64, 23, 314],
                operation: Operation::Add
            }
            .calculate(),
            401
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(operation("+bc"), Ok(("bc", Operation::Add)));
        assert_eq!(operation("*bc"), Ok(("bc", Operation::Multiply)));
        assert_eq!(
            parse_oper("*   +"),
            Ok(("", vec![Operation::Multiply, Operation::Add]))
        );
        assert_eq!(parse_ops("123   456"), Ok(("", vec![123, 456])));
    }
}
