use crate::{exchange, parse_input, partner, spin, Move};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, size: i32) -> Result<String> {
    let (_, moves) = parse_input(input)?;
    let mut programs: Vec<char> = vec![];
    for x in 0u8..size as u8 {
        programs.push((x + 97u8) as char);
    }

    for m in moves {
        match m {
            Move::Spin(x) => spin(x, &mut programs),
            Move::Exchange(x, y) => exchange(x, y, &mut programs),
            Move::Partner(a, b) => partner(a, b, &mut programs),
        }
    }

    let result: String = programs.iter().collect();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "s1,x3/4,pe/b";
        assert_eq!("baedc", process(input, 5)?);
        Ok(())
    }

    #[test]
    fn test_spin() {
        let mut input = vec!['a', 'b', 'c', 'd', 'e'];
        spin(3, &mut input);
        assert_eq!(vec!['c', 'd', 'e', 'a', 'b'], input);
    }
}
