use crate::{find_bottom, parse_input};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut tower_map) = parse_input(input)?;
    let bottom = find_bottom(&mut tower_map);
    Ok(bottom.to_string())
}

/*
need to find which tower has nothing underneath it.

assume a tower has nothing underneath
loop over each tower. if a disk is in the HashSet then remove it
at the end there should only be one disk left.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!("tknk", process(input)?);
        Ok(())
    }
}
