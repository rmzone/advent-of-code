use crate::{parse_input, scenic_score};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let trees = parse_input(input);

    info!("{}", &trees);

    let size = trees.size();
    let mut score = 0;

    for y in 0..size {
        for x in 0..size {
            let sc = scenic_score(x, y, &trees);
            if sc > score {
                score = sc;
            }
        }
    }

    Ok(score.to_string())
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
