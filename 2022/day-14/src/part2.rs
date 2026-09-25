use crate::{parse_input, Cave};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, paths) = parse_input(input)?;
    info!("{:?}", &paths);

    let mut cave = Cave::new(&paths);
    cave.add_bottom();
    info!("{}\n", &cave);

    let mut particles_dropped = 1;

    while cave.drop_sand() {
        particles_dropped += 1;
    }

    info!("{}\n", &cave);

    Ok(particles_dropped.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!("93", process(input)?);
        Ok(())
    }
}
