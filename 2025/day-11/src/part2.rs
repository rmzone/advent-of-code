use std::collections::{BTreeMap, HashMap, HashSet};
use common::custom_error::Result;
use crate::{count_paths, parse_input};

// Only two groups of paths:
// svr -> fft -> dac --> out
// svr -> dac -> fft --> out (gave 0 as the answer)
// The solution would be the product of (svr -> fft) * (fft -> dac) * (dac --> out)
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, connections) = parse_input(input)?;
    let mut result = count_paths(&connections, &mut BTreeMap::new(), "svr", "fft");
    result *= count_paths(&connections, &mut BTreeMap::new(), "fft", "dac");
    result *= count_paths(&connections, &mut BTreeMap::new(), "dac", "out");

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
