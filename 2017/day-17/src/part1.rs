use crate::parse_input;
use common::custom_error::Result;
use log::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, steps) = parse_input(input)?;
    let mut buffer = vec![0];
    let mut index = 0usize;

    info!("{} - {:?}", 0, buffer);

    for v in 1..2018 {
        index = (index + steps as usize) % buffer.len();

        if index + 1 >= buffer.len() {
            buffer.push(v);
        } else {
            buffer.insert(index + 1, v as i32);
        }

        index += 1;
        // info!("{} - {:?}", v, buffer);
    }

    let next = buffer[index + 1]; // assume it is not at the end

    Ok(next.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3";
        assert_eq!("638", process(input)?);
        Ok(())
    }
}
