use glam::IVec3;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Particle {
    pub id: i32,
    pub position: IVec3,
    pub velocity: IVec3,
    pub acceleration: IVec3,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Particle>> {
    let (input, mut particles) = separated_list1(line_ending, parse_particle).parse(input)?;
    let mut id = 0;
    for particle in particles.iter_mut() {
        particle.id = id;
        id += 1;
    }

    Ok((input, particles))
}

fn parse_particle(input: &str) -> IResult<&str, Particle> {
    let (input, _) = tag("p=")(input)?;
    let (input, position) = parse_coordinate(input)?;
    let (input, _) = tag(", v=")(input)?;
    let (input, velocity) = parse_coordinate(input)?;
    let (input, _) = tag(", a=")(input)?;
    let (input, acceleration) = parse_coordinate(input)?;

    Ok((
        input,
        Particle {
            id: 0,
            position,
            velocity,
            acceleration,
        },
    ))
}

fn parse_coordinate(input: &str) -> IResult<&str, IVec3> {
    let (input, (x, _, y, _, z)) = delimited(
        tag("<"),
        (
            complete::i32,
            tag(","),
            complete::i32,
            tag(","),
            complete::i32,
        ),
        tag(">"),
    )
    .parse(input)?;
    Ok((input, IVec3::new(x, y, z)))
}
