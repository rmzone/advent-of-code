use common::custom_error::Result;
use tracing::info;
use crate::{chunk, parse_input, to_array, START_PATTERN};
/*

#./..
.#/..
../.#
../#.

fn's:
read rules (pre bake size so it can be rapidly elimanated)
calculate grid sizes
gen all rotations/flips
detect match and apply match (always apply unfliped/rotated)
split
merge


*/

#[tracing::instrument(skip(input))]
pub fn process(input: &str, iterations: usize) -> Result<String> {
    let (_, rules) = parse_input(input)?;
    info!("{:?}", rules);

    let mut image = START_PATTERN.to_string();

    let image_array = to_array(&image);
    info!("{:?}", image_array);

    for _ in 0..iterations {
        // break into chunks based on size
        let chunks = chunk(&image_array, &rules);

        // for each chunk find rule
        // apply rule
        // merge back new chunks
    }

    let count = image.chars().filter(|&c| c == '#').count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        assert_eq!("12", process(input, 2)?);
        Ok(())
    }
}
