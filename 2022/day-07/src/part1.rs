use crate::{calculate_sizes, parse_input, Directive, Node};
use common::custom_error::Result;
use std::collections::BTreeMap;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, directives) = parse_input(input.trim())?;

    info!("{:?}", directives);

    let (_, sizes) = directives
        .iter()
        .fold((vec![], BTreeMap::new()), calculate_sizes);

    info!("{:?}", sizes);

    // get a list of all dir and their sizes
    // find all by adding all combos <= 100_000
    let result = sizes
        .iter()
        .filter(|&(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<usize>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        assert_eq!("95437", process(input)?);
        Ok(())
    }
}
