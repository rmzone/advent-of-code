use crate::{parse_input, Equation};
use common::custom_error::{Error, Result};
use itertools::Itertools;

const OPERATORS: [char; 2] = ['*', '+'];

pub fn process(input: &str) -> Result<String, Error> {
    let (_, equations) = parse_input(input).expect("should parse");

    let mut result = 0;

    for equation in equations {
        if is_valid(&equation) {
            result += equation.test_value;
        }
    }

    Ok(result.to_string())
}

fn is_valid(equation: &Equation) -> bool {
    // generate all permutations of + and *. n-1 * 2
    let length = equation.operators.iter().len() - 1;
    let perms = (0..length).map(|_| OPERATORS).multi_cartesian_product();

    for candidate in perms {
        // println!("candidate: {:?}", &candidate);
        let mut acc = equation.operators[0];
        for i in 0..length {
            // get each pair and operator
            let b = equation.operators[i + 1];
            let c = candidate[i];

            // perform math
            acc = match c {
                '+' => acc + b,
                '*' => acc * b,
                _ => panic!("invalid operator"),
            };
        }

        // if equal to test_value return true
        if acc == equation.test_value {
            println!("{:?} {:?}", equation, &candidate);

            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(
            is_valid(&Equation {
                test_value: 292,
                operators: vec![11, 6, 16, 20]
            }),
            true
        );
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
