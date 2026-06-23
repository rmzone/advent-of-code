use crate::{START_PATTERN, enhance, parse_input, to_square_grid};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, iterations: usize) -> Result<String> {
    let (_, rules) = parse_input(input)?;
    info!("{:?}", &rules);
    let mut image = to_square_grid(START_PATTERN);
    info!("{}", &image);

    for _ in 0..iterations {
        image = enhance(&image, &rules);
    }

    let count = image.values().iter().filter(|&c| c == &'#').count();

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
