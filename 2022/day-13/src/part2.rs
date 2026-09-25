use crate::{parse_input, Packet, Pair};
use common::custom_error::Result;
use nom::combinator::flat_map;
use std::cmp::Ordering;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let input = input.to_string() + "\n[[2]]\n[[6]]\n";
    let (_, pairs) = parse_input(&input)?;

    // flatten
    let mut packets = pairs
        .into_iter()
        .map(|p| vec![p.left, p.right])
        .flatten()
        .collect::<Vec<_>>();

    packets.sort();
    info!("{}", packets.len());

    // divider packets to find
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    let result = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| packet == &&divider1 || packet == &&divider2)
        .map(|(idx, _)| idx + 1)
        .product::<usize>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!("140", process(input)?);
        Ok(())
    }
}
