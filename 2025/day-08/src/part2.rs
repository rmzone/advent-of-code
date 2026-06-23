use crate::parse_input;
use common::custom_error::Result;
use glam::IVec3;
use itertools::Itertools;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, points) = parse_input(input)?;
    let mut result = 0u64;

    // place each point in its own group
    let mut circuits: HashMap<usize, Vec<IVec3>> =
        points
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, p)| {
                acc.insert(i, vec![*p]);
                acc
            });

    for (p1, p2, dist) in points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    {
        //  info!("closest_pair({}, {}, {})", p1, p2, dist);

        // need to merge pairs here
        // the pair is in either the same or different group
        let &p1_key = circuits // TODO: merge into function
            .iter()
            .find(|&(_, v)| v.contains(&p1))
            .map(|(k, _)| k)
            .unwrap_or(&usize::MAX);
        let &p2_key = circuits // TODO: merge into function
            .iter()
            .find(|&(_, v)| v.contains(&p2))
            .map(|(k, _)| k)
            .unwrap_or(&usize::MAX);

        if p1_key != p2_key {
            // perform merge (items in key2 to key1)
            let circuit2_p2 = circuits.get(&p2_key).unwrap().clone();
            let circuits_p1 = circuits.get_mut(&p1_key).unwrap();

            for c in circuit2_p2 {
                circuits_p1.push(c);
            }

            circuits.remove(&p2_key);
        }

        // check if we only have one group
        if circuits.len() <= 1 {
            result = p1.x as u64 * p2.x as u64;
            break;
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!("25272", process(input)?);
        Ok(())
    }
}
