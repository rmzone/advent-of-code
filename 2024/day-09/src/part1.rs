use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let mut blocks = process_input(input);
    print(&blocks);

    // do compaction
    let mut free_index = 0;
    let mut index = blocks.len() - 1;

    while free_index < index {
        if blocks[index].is_some() {
            if blocks[free_index].is_some() {
                free_index += 1;
                continue;
            } else {
                blocks.swap(index, free_index);
                index -= 1;
                free_index += 1;
                print(&blocks);
            }
        } else {
            index -= 1;
            continue;
        }
    }

    // calculate checksum
    let mut result = 0u64;

    for (i, block) in blocks.iter().enumerate() {
        result += match block {
            Some(block) => (block * i as u32) as u64,
            None => 0,
        }
    }

    Ok(result.to_string())
}

fn print(blocks: &Vec<Option<u32>>) {
    for block in blocks {
        match block {
            Some(i) => print!("{}", i),
            None => print!("."),
        }
    }
    println!();
}

fn process_input(input: &str) -> Vec<Option<u32>> {
    let mut blocks: Vec<Option<u32>> = vec![];
    let mut mode = true;

    let mut id = 0;

    for ch in input.chars() {
        let num = ch.to_string().parse::<u32>().unwrap();

        for _ in 0..num {
            let block = match mode {
                true => Some(id),
                false => None,
            };

            blocks.push(block);
        }

        if mode {
            id += 1;
        }

        mode = !mode;
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
