use crate::{START_PATTERN, enhance, parse_input, to_square_grid};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, iterations: usize) -> Result<String> {
    let (_, rules) = parse_input(input)?;
    let mut image = to_square_grid(START_PATTERN);

    for _ in 0..iterations {
        image = enhance(&image, &rules);
    }

    let count = image.values().iter().filter(|&c| c == &'#').count();

    Ok(count.to_string())
}
