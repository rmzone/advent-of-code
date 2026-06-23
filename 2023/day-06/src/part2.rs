use common::custom_error::AocError;
use nom::character::complete::space0;
use nom::multi::fold_many1;
use nom::sequence::{preceded, terminated};
use nom::{
    character::complete::{self, line_ending, space1},
    sequence::tuple,
    IResult,
};

fn parse_set(input: &str) -> IResult<&str, Vec<u64>> {
    fold_many1(
        terminated(complete::u64, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

#[derive(Debug)]
struct Game {
    time: u64,
    distance: u64,
}

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

    let mut time = "".to_string();
    let mut distance = "".to_string();

    for i in 0..times.len() {
        time.push_str(format!("{}", times[i]).as_str());
        distance.push_str(format!("{}", distances[i]).as_str());
    }

    games.push(Game {
        time: time.parse::<u64>().unwrap(),
        distance: distance.parse::<u64>().unwrap(),
    });

    Ok((input, games))
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, games) = parse_games(input).unwrap();

    let mut result: u64 = 1;

    for game in games {
        let mut score = 0;
        for t in 0..game.time {
            let distance = (game.time - t) * t;
            if distance > game.distance {
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

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
