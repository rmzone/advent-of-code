use crate::parse_input;
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, mut computer) = parse_input(input).expect("Should parse!");

    loop {
        let should_halt = computer.execute();
        if should_halt {
            break;
        }
    }

    // dbg!(&computer);

    Ok(computer.get_output())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Computer;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example1() -> Result<()> {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);

        computer.execute();

        assert_eq!(1, computer.reg_b);
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        assert_eq!("0,1,2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example4() -> Result<()> {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);

        computer.execute();

        assert_eq!(26, computer.reg_b);
        Ok(())
    }

    #[test]
    fn test_example5() -> Result<()> {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);

        computer.execute();

        assert_eq!(44354, computer.reg_b);
        Ok(())
    }
}
