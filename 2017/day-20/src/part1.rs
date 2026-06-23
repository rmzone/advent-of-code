use crate::parse_input;
use common::custom_error::Result;
use glam::IVec3;
use log::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut particles) = parse_input(input)?;

    info!("{:?}", particles);

    for _ in 0..1000 {
        // must be a better way to do instead of just an arbitrary number.
        for p in particles.iter_mut() {
            // advance particle
            p.velocity += p.acceleration;
            p.position += p.velocity;
        }
    }

    let result = particles
        .iter()
        .min_by(|&a, &b| {
            a.position
                .manhattan_distance(IVec3::ZERO)
                .cmp(&b.position.manhattan_distance(IVec3::ZERO))
        })
        .unwrap()
        .id;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        assert_eq!("0", process(input)?);
        Ok(())
    }
}
