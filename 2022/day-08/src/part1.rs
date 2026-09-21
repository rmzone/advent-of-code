use crate::{is_visible, parse_input};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let trees = parse_input(input);

    info!("{}", &trees);

    let size = trees.size();
    let mut visible = 0;

    for y in 0..size {
        for x in 0..size {
            if is_visible(x, y, &trees) {
                visible += 1;
            }
        }
    }

    Ok(visible.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
