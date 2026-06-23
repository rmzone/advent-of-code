use common::custom_error::AocError;
use std::collections::HashMap;
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

    let mut current = "AAA";
    let mut count = 0;

    while current != "ZZZ" {
        let instruction = instructions[count % instructions.len()];
        let node = map.get(&current).unwrap();
        match instruction {
            'L' => current = node.left,
            'R' => current = node.right,
            _ => panic!("should not get here"),
        }
        count += 1;
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}

#[test]
fn test_process2() -> miette::Result<()> {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!("6", process(input)?);
    Ok(())
}
