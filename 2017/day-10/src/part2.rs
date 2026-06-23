use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, list_length: i32) -> Result<String> {
    let mut lengths = input.chars().map(|c| c as i32).collect::<Vec<i32>>();
    let mut suffix = vec![17, 31, 73, 47, 23];
    lengths.append(&mut suffix);

    let mut list: Vec<i32> = (0..list_length).collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in lengths.iter() {
            // Reverse the order of that length of elements in the list, starting with the element at the current position.
            let list_copy = list.clone();
            for index in 0..*length {
                let source = ((current_position + index) % list_length) as usize;
                let destination = ((current_position + length - index - 1) % list_length) as usize;

                // info!("{} <> {}", &source, &destination);

                if source != destination {
                    // swap
                    list[destination] = list_copy[source];
                }
            }

            // Move the current position forward by that length plus the skip size.
            current_position += (length + skip_size as i32) % list_length;

            // Increase the skip size by one.
            skip_size += 1;
        }
    }

    // info!("{:?}", &list);
    let result = hash_list(&list);

    Ok(result.to_string())
}

// fold(0, |acc, row| acc + find_even_divisor(&row));
fn hash_list(list: &Vec<i32>) -> String {
    let mut result = String::new();
    let chunks: Vec<&[i32]> = list.chunks(16).collect();

    for chunk in chunks {
        let temp = chunk.iter().fold(0, |acc, x| acc ^ x);
        result.push_str((format!("{:02x}", temp).as_str()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "";
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", process(input, 256)?);

        let input = "AoC 2017";
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", process(input, 256)?);

        let input = "1,2,3";
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", process(input, 256)?);

        let input = "1,2,4";
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", process(input, 256)?);
        Ok(())
    }
}
