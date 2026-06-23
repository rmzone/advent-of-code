use common::custom_error::{Error, Result};

#[derive(Debug)]
struct Block {
    pub index: usize,
    pub size: usize,
    pub id: Option<usize>,
}

pub fn process(input: &str) -> Result<String, Error> {
    let high_index: usize = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();
    let (mut blocks_map, mut free_blocks) = process_input(input);

    // print(&blocks_map, &free_blocks, high_index);
    // print(&blocks);
    // println!("{:?}", &free_blocks);
    // println!(
    //     "HighIndex: {:?}, Length {:?}, {:?}",
    //     high_index,
    //     &blocks_map.iter().len(),
    //     &free_blocks.iter().len()
    // );

    // println!("{:?}", &free_blocks);

    // do compaction
    for block in blocks_map.iter_mut().rev() {
        // println!("Block: {:?}", &block);

        // find an empty slot
        let free = free_blocks
            .iter_mut()
            .find(|b| b.size != 0 && b.size >= block.size && b.index < block.index);

        match free {
            Some(free) => {
                // perform a swap
                //  println!("Before Block: {:?}, Free: {:?}", &block, &free);
                block.index = free.index;
                free.index += block.size;
                free.size -= block.size;
                // println!("{:?}", &free_blocks);
                //println!("After Block: {:?}, Free: {:?}", &block, &free);
            }
            None => {
                continue;
            }
        }

        // print(&blocks);
    }

    // println!("{:?}", &blocks_map);
    // for block in blocks_map.iter() {
    //     println!("Block: {:?}", &block);
    // }
    // for block in free_blocks.iter() {
    //     println!("Free Block: {:?}", &block);
    // }

    // calculate checksum
    let mut result = 0u64;
    //
    for block in blocks_map.iter() {
        let id = block.id.unwrap_or(0);

        for i in 0..block.size {
            result += (id * (block.index + i)) as u64;
        }

        // println!("Block: {:?} => {:?}", &block, &result);
    }

    // println!("{:?}", &free_blocks);

    Ok(result.to_string())
}

fn process_input(input: &str) -> (Vec<Block>, Vec<Block>) {
    let mut blocks_map: Vec<Block> = vec![];
    let mut free_blocks: Vec<Block> = vec![];

    let mut mode = true;
    let mut id = 0;
    let mut index = 0;

    for ch in input.chars() {
        let num = ch.to_string().parse::<u32>().unwrap() as usize;

        if mode {
            blocks_map.push(Block {
                index,
                size: num,
                id: Some(id),
            });
        } else {
            free_blocks.push(Block {
                index,
                size: num,
                id: None,
            });
        }

        if mode {
            id += 1;
        }

        mode = !mode;
        index += num;
    }

    // println!("{:?}", &blocks_map);
    // println!("{:?}", &free_blocks);
    // println!("{}", &free_blocks.len());

    (blocks_map, free_blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }

    // 0         1         2         3         4
    // 012345678901234567890123456789012345678901
    // 00...111...2...333.44.5555.6666.777.888899
    // 0099.111...2...333.44.5555.6666.777.8888..
    // 0099.1117772...333.44.5555.6666.....8888..
    // 0099.111777244.333....5555.6666.....8888..
    // 00992111777.44.333....5555.6666.....8888..
}
