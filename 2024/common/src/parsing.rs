use glam::IVec2;
use nom::{character::complete::one_of, IResult};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;


pub fn token<'a>(
    tokens: &str,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, (IVec2, char)> + use<'a, '_> {
    move |input| {
        let (input, pos) = position(input)?;
        let x = pos.get_column() as i32 - 1;
        let y = pos.location_line() as i32 - 1;
        let (input, c) = one_of(tokens)(input)?;

        Ok((input, (IVec2::new(x, y), c)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let input = Span::new("ABC");
        let (input, item) = token("A")(input).unwrap();

        assert_eq!(input.to_string(), "BC");
        assert_eq!(item, (IVec2::new(0, 0), 'A'));
    }
}
