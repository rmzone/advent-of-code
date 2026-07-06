pub mod part1;
pub mod part2;

pub fn knot_hash(lengths: &Vec<i32>) -> Vec<i32> {
    let list_length: i32 = 256;
    let mut suffix = vec![17, 31, 73, 47, 23];
    let mut lengths = lengths.clone();
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

    list
}

pub fn hash_list(list: &Vec<i32>) -> String {
    let mut result = String::new();
    let chunks: Vec<&[i32]> = list.chunks(16).collect();

    for chunk in chunks {
        let temp = chunk.iter().fold(0, |acc, x| acc ^ x);
        result.push_str((format!("{:02x}", temp).as_str()));
    }

    result
}
