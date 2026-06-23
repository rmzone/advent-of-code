use crate::process_input;
use common::custom_error::Result;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, points) = process_input(input)?;

    // let min_x = points.iter().min_by_key(|&p| p.x).unwrap();
    // let max_x = points.iter().max_by_key(|&p| p.x).unwrap();
    // let min_y = points.iter().min_by_key(|&p| p.y).unwrap();
    // let max_y = points.iter().max_by_key(|&p| p.y).unwrap();
    //
    // println!("min:({},{}), max:({},{})", min_x, min_y, max_x , max_y);
    //
    // let extreme_points: Vec<I64Vec3> = vec![*min_x, *min_y, *max_x, *max_y];
    //
    // let result = extreme_points
    //     .iter()
    //     .tuple_combinations()
    //     .map(|(a, b)| area(a, b))
    //     .sorted_by(|a, b| b.cmp(&a))
    //     .take(1)
    //     .collect::<Vec<_>>();

    let result = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .expect("no points found");

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
