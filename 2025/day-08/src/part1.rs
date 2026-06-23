use crate::parse_input;
use common::custom_error::Result;
use glam::IVec3;
use itertools::Itertools;
use std::collections::HashMap;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, c: usize) -> Result<String> {
    let (_, points) = parse_input(input)?;

    // setup pairs, and initial junction boxes
    let mut circuits: HashMap<IVec3, i32> = HashMap::new();
    let mut circuit_id = 0;
    for b in &points {
        circuits.insert(*b, circuit_id);
        circuit_id += 1;
    }

    // find c closest pairs
    for (p1, p2, dist) in closest_pair(&points, c) {
        info!("closest_pair({}, {}, {})", p1, p2, dist);
        group_into_circuits(&mut circuits, &(p1, p2));
    }

    let mut group_counts: HashMap<i32, usize> = HashMap::new();
    for (&item, &group_id) in circuits.iter() {
        println!("GroupID: {}, {:?}", group_id, item);
        *group_counts.entry(group_id).or_insert(0) += 1;
    }

    let mut result = 1;
    for (group_id, count) in group_counts.iter().sorted_by(|a, b| b.1.cmp(a.1)).take(3) {
        println!("Group {}: {} members", group_id, count);
        result *= count;
    }

    Ok(result.to_string())
}

// https://en.wikipedia.org/wiki/Closest_pair_of_points_problem
// https://codelucky.com/closest-pair-problem/
// https://stackoverflow.com/questions/35617048/closest-pair-of-points-in-3-dimensions-divide-and-conquer
// https://www.geeksforgeeks.org/dsa/closest-pair-of-points-using-divide-and-conquer-algorithm/#expected-approach-using-divide-and-conquer-on-logn2-time-and-on-space
fn closest_pair(points: &Vec<IVec3>, count: usize) -> Vec<(IVec3, IVec3, f32)> {
    let mut result: Vec<(IVec3, IVec3, f32)> = Vec::new();

    for (a, b, d) in points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .take(count)
    {
        result.push((a.clone(), b.clone(), d));
    }

    result
}

fn group_into_circuits(groups: &mut HashMap<IVec3, i32>, pair: &(IVec3, IVec3)) {
    let &g1 = groups.get(&pair.0).unwrap();
    let &g2 = groups.get(&pair.1).unwrap();

    if g1 != g2 {
        info!("merges groups {} {}", g1, g2);
        for value in groups.values_mut() {
            if *value == g2 {
                *value = g1;
            }
        }
    } else {
        info!("same group so nothing to do");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

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
        assert_eq!("40", process(input, 10)?);
        Ok(())
    }
}
