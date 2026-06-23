use common::custom_error::AocError;
use std::collections::HashMap;
use std::ops::Deref;
use substring::Substring;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_node(input: &str) -> (&str, Node) {
    let id = input.substring(0, 3);
    let left = input.substring(7, 10);
    let right = input.substring(12, 15);

    // dbg!(&id, &left, &right);

    (id, Node { left, right })
}

fn parse_map(input: &str) -> (Vec<char>, HashMap<&str, Node>) {
    let mut lines = input.lines();
    let mut map: HashMap<&str, Node> = HashMap::new();

    let instructions = lines.next().unwrap();
    lines.next().unwrap(); // skip line

    while let Some(line) = lines.next() {
        let (id, node) = parse_node(&line);
        // dbg!(&id, &node);
        map.insert(id, node);
    }

    (instructions.chars().collect(), map)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (instructions, map) = parse_map(input);
    let mut keys: Vec<_> = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|u| u.deref())
        .collect();
    let mut count = 0;
    let mut steps: [u64; 6] = [0; 6];

    while keys.len() > 0 {
        let instruction = instructions[count % instructions.len()];

        for i in 0..keys.len() {
            let current = keys[i];
            let node = map.get(&current).unwrap();
            match instruction {
                'L' => keys[i] = node.left,
                'R' => keys[i] = node.right,
                _ => panic!("should not get here"),
            }
            steps[i] += 1;
        }

        count += 1;
        keys.retain(|key| !key.ends_with("Z"));
    }

    dbg!(&steps);

    let mut result = 1;

    for num in steps {
        if num == 0 {
            continue;
        }
        result = num::integer::lcm(result, num);
    }

    Ok(result.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
