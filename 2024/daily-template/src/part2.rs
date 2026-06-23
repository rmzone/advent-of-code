use common::custom_error::{Result, Error};

pub fn process(_input: &str) -> Result<String, Error> {
    todo!("part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}