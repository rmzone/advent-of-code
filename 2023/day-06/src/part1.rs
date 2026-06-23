use common::custom_error::AocError;
use nom::character::complete::space0;
use nom::multi::fold_many1;
use nom::sequence::{preceded, terminated};
use nom::{
    character::complete::{self, line_ending, space1},
    sequence::tuple,
    IResult,
};

fn parse_set(input: &str) -> IResult<&str, Vec<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

#[derive(Debug)]
struct Game {
    time: u32,
    distance: u32,
}

#[tracing::instrument(skip(input))]
fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, times) = preceded(
        tuple((nom::bytes::complete::tag("Time: "), space1)),
        terminated(parse_set, line_ending),
    )(input)
    .unwrap();

    let (input, distances) = preceded(
        tuple((nom::bytes::complete::tag("Distance: "), space1)),
        terminated(parse_set, line_ending),
    )(input)
    .unwrap();

    let mut games: Vec<Game> = vec![];

    for i in 0..times.len() {
        games.push(Game {
            time: times[i],
            distance: distances[i],
        })
    }

    Ok((input, games))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, games) = parse_games(input).unwrap();

    let mut result: u32 = 1;

    for game in games {
        let mut score = 0;
        for time in 0..game.time {
            let speed = (game.time - time) * time;
            if speed > game.distance {
                score += 1;
            }
        }

        result *= score;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
