use crate::parse_input;
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, list_length: i32) -> Result<String> {
    let (_, lengths) = parse_input(input)?;
    let mut list: Vec<i32> = (0..list_length).collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    for length in lengths {
        // Reverse the order of that length of elements in the list, starting with the element at the current position.
        let list_copy = list.clone();
        for index in 0..length {
            let source = ((current_position + index) % list_length) as usize;
            let destination = ((current_position + length - index - 1) % list_length) as usize;

            info!("{} <> {}", &source, &destination);

            if source != destination {
                // swap
                list[destination] = list_copy[source];
            }
        }

        // Move the current position forward by that length plus the skip size.
        current_position += (length + skip_size as i32) % list_length;

        // Increase the skip size by one.
        skip_size += 1;

        info!("{} {:?}", &current_position, &list);
    }

    let result = list[0] * list[1];

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3,4,1,5";
        assert_eq!("12", process(input, 5)?);
        Ok(())
    }
}
