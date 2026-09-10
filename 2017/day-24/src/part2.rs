use crate::{Port, calculate_strength, my_display, parse_input};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, ports) = parse_input(input)?;
    let mut max_strength = 0;
    let mut max_length = 0;

    build_bridge(0, &ports, &Vec::new(), &mut max_strength, &mut max_length);

    Ok(max_strength.to_string())
}

fn build_bridge(
    start: i32,
    ports: &Vec<Port>,
    current_bridge: &Vec<Port>,
    current_max: &mut i32,
    current_length: &mut i32,
) {
    let current_strength = calculate_strength(&current_bridge);
    let length = current_bridge.len() as i32;
    if current_strength > *current_max && length >= *current_length {
        *current_max = current_strength;
        *current_length = length;
    }

    // find next matching possible ports
    let neighbors = ports
        .iter()
        .filter(|p| p.type1 == start || p.type2 == start)
        .collect::<Vec<&Port>>();
    for neighbor in neighbors {
        if current_bridge.contains(&neighbor) {
            // skip if the neighbor is already in the list
            continue;
        }

        let mut next = current_bridge.clone();
        next.push(neighbor.clone());

        if neighbor.type1 == start {
            build_bridge(neighbor.type2, &ports, &next, current_max, current_length);
        } else {
            build_bridge(neighbor.type1, &ports, &next, current_max, current_length);
        }
    }

    info!("{}", my_display(current_bridge));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!("19", process(input)?);
        Ok(())
    }
}
