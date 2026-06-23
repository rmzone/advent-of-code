use crate::parse_input;
use common::custom_error::Result;
use log::info;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut particles) = parse_input(input)?;

    info!("{:?}", particles);

    for _ in 0..1000 {
        // must be a better way to do instead of just an arbitrary number.
        // move all the particles
        for p in particles.iter_mut() {
            // advance particle
            p.velocity += p.acceleration;
            p.position += p.velocity;
        }

        // remove every particle occupying a collided position
        let mut position_counts = HashMap::new();
        for particle in &particles {
            *position_counts.entry(particle.position).or_insert(0_usize) += 1;
        }

        particles.retain(|particle| position_counts[&particle.position] == 1);
    }

    Ok(particles.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>
";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
