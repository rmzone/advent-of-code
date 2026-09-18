use crate::{parse_input, Shape, Strategy};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, rounds) = parse_input(input)?;
    info!("{:?}", &rounds);
    let score = rounds
        .iter()
        .map(|u| {
            let our_move = match u.player2 {
                Shape::Rock => {
                    // lose
                    match u.player1 {
                        Shape::Rock => Shape::Scissors,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissors => Shape::Paper,
                    }
                }
                Shape::Paper => {
                    // draw
                    u.player1
                }
                Shape::Scissors => {
                    // win
                    match u.player1 {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissors,
                        Shape::Scissors => Shape::Rock,
                    }
                }
            };

            Strategy {
                player1: u.player1,
                player2: our_move,
            }
            .calculate_score()
        })
        .sum::<u32>();

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "A Y
B X
C Z";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
